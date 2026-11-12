#!/usr/bin/env python3

# Print the epoch estimates for proposals/mainnet/disable-masp-rewards/data.json
# based on target dates.
#
# NOTE: the chain must be live when this runs; while the chain is halted,
# epochs are frozen and the estimates will be wrong.
#
# Usage:
#   python3 scripts/estimate-proposal-epochs.py --voting-start 2026-11-18 \
#       --voting-end 2026-11-25 --activation 2026-11-27

import argparse
import base64
import json
import sys
import urllib.parse
import urllib.request
from datetime import datetime, timedelta, timezone

RPC = "https://rpc.namada.net"

GOV_ADDRESS = "tnam1q5qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqrw33g6"
PARAMS_ADDRESS = "tnam1qsqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxdl54l"


def abci_query(path: str) -> bytes:
    """Query the namada node and return the raw (borsh) value bytes."""
    url = RPC + "/abci_query?path=" + urllib.parse.quote(json.dumps(path))
    with urllib.request.urlopen(url, timeout=10) as resp:
        result = json.load(resp)
    response = result["result"]["response"]
    if response.get("code", 0) != 0 or response.get("value") is None:
        raise RuntimeError(
            f"abci_query failed for {path!r}: {response.get('log', '')!r}"
        )
    return base64.b64decode(response["value"])


def read_u64(path: str) -> int:
    value = abci_query(path)
    return int.from_bytes(value[:8], "little")


def jsonrpc_rpc(method: str, params: dict):
    body = json.dumps(
        {"jsonrpc": "2.0", "id": -1, "method": method, "params": params}
    ).encode()
    req = urllib.request.Request(
        RPC,
        data=body,
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=10) as resp:
        result = json.load(resp)
    if "error" in result:
        raise RuntimeError(f"{method} failed: {result['error']}")
    return result["result"]


def main():
    parser = argparse.ArgumentParser(
        description="Estimate proposal epochs from target dates."
    )
    parser.add_argument(
        "--voting-start", required=True, help="Voting start date (YYYY-MM-DD, UTC)"
    )
    parser.add_argument(
        "--voting-end", required=True, help="Voting end date (YYYY-MM-DD, UTC)"
    )
    parser.add_argument(
        "--activation", required=True, help="Activation date (YYYY-MM-DD, UTC)"
    )
    args = parser.parse_args()

    dates = {
        name: datetime.strptime(value, "%Y-%m-%d").replace(tzinfo=timezone.utc)
        for name, value in [
            ("voting_start_epoch", args.voting_start),
            ("voting_end_epoch", args.voting_end),
            ("activation_epoch", args.activation),
        ]
    }

    # --- Read live chain state ------------------------------------------------
    current_epoch = read_u64("/shell/epoch")

    # Epoch duration: EpochDuration { min_num_of_blocks: u64, min_duration (secs): u64 }
    epoch_duration_bytes = abci_query(f"/shell/value/#{PARAMS_ADDRESS}/epoch_duration")
    min_duration_secs = int.from_bytes(epoch_duration_bytes[8:16], "little")
    epoch_len = timedelta(seconds=min_duration_secs)

    # Anchor "now" to the actual start time of the current epoch: read the
    # first block height of the current epoch, then the block header time.
    first_height = read_u64("/shell/first_block_height_of_current_epoch")
    block_result = jsonrpc_rpc("block", {"height": str(first_height)})
    epoch_start = datetime.fromisoformat(
        block_result["block"]["header"]["time"].replace("Z", "+00:00")
    )

    # Detect a time gap between the chain head and wall clock (the chain may
    # be halted, in which case epochs are frozen and the estimates below
    # would only be valid if/when the chain is live again).
    status = jsonrpc_rpc("status", {})
    latest_block_time = datetime.fromisoformat(
        status["sync_info"]["latest_block_time"].replace("Z", "+00:00")
    )
    head_gap = datetime.now(timezone.utc) - latest_block_time

    # Governance parameters (u64 LE each)
    min_voting_period = read_u64(f"/shell/value/#{GOV_ADDRESS}/min_period")
    min_grace_epochs = read_u64(f"/shell/value/#{GOV_ADDRESS}/min_grace_epochs")
    max_period = read_u64(f"/shell/value/#{GOV_ADDRESS}/max_period")
    max_latency = read_u64(f"/shell/value/#{GOV_ADDRESS}/max_latency")

    # --- Compute epochs -------------------------------------------------------
    # Epoch boundaries: the epoch containing time T started at
    # epoch_start + floor((T - epoch_start) / epoch_len) * epoch_len, so the
    # epoch index at T is current_epoch + that floor value.
    def epoch_at(target: datetime) -> int:
        if target < epoch_start:
            raise RuntimeError(f"{target} is before the current epoch start")
        delta = target - epoch_start
        return current_epoch + int(delta.total_seconds()) // int(
            epoch_len.total_seconds()
        )

    epochs = {name: epoch_at(dt) for name, dt in dates.items()}
    now_epoch = epoch_at(datetime.now(timezone.utc))

    print(f"Current epoch:               {current_epoch}")
    print(f"Current epoch started at:    {epoch_start.isoformat()}")
    print(f"Epoch duration:              {epoch_len}")
    print()
    if head_gap > epoch_len:
        print(
            f"WARNING: chain head is {head_gap} behind wall clock "
            f"(halted?); epochs are frozen at {current_epoch}"
        )
        print(f"estimated_epoch_now:    {now_epoch} (if the chain were live)")
    print(
        f"submit_before_epoch:    {epochs['voting_start_epoch'] - max_latency} "
        f"(voting_start - max_latency {max_latency})"
    )
    print(f"voting_start_epoch:     {epochs['voting_start_epoch']}")
    print(f"voting_end_epoch:       {epochs['voting_end_epoch']}")
    print(f"activation_epoch:       {epochs['activation_epoch']}")
    print()

    # --- Validate against on-chain governance parameters ----------------------
    errors = []

    span = epochs["voting_end_epoch"] - epochs["voting_start_epoch"]
    if span < min_voting_period:
        errors.append(
            f"voting period too short: {span} < min {min_voting_period} epochs"
        )

    grace = epochs["activation_epoch"] - epochs["voting_end_epoch"]
    if grace < min_grace_epochs:
        errors.append(
            f"grace period too short: {grace} < min {min_grace_epochs} epochs"
        )

    total = epochs["activation_epoch"] - epochs["voting_start_epoch"]
    if total > max_period:
        errors.append(f"proposal too long: {total} > max {max_period} epochs")

    # max_latency: voting_start must be within N epochs of "now"
    latency = epochs["voting_start_epoch"] - now_epoch
    if latency < 0:
        errors.append(
            f"voting start epoch {epochs['voting_start_epoch']} is in the past"
        )
    elif latency > max_latency:
        print(
            f"WARNING: voting start is {latency} epochs ahead; the proposal "
            f"must be submitted within {max_latency} epochs of voting_start "
            f"(i.e. around epoch {epochs['voting_start_epoch'] - max_latency})"
        )

    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
