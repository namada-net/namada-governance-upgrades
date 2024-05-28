from typing import Dict
from datetime import datetime
import argparse
import json
import os

GOVERNANCE_GENESIS_PARAMETERS = {
    "max_proposal_code_size": 300000,
    "min_proposal_voting_period": 3,
    "max_proposal_period": 27,
    "max_proposal_content_size": 10000,
    "min_proposal_grace_epochs": 2,
    "max_proposal_latency": 30
}

def validate_data(data: Dict[str, str]) -> bool:
    start_epoch = data['voting_start_epoch']
    end_epoch = data['voting_end_epoch']
    activation_epoch = data['grace_epoch']
    proposal_content = len(json.dumps(data))
    wasm_size = os.path.getsize(data['wasm_path']) / 1024 # in kibibytes

    if end_epoch <= start_epoch or activation_epoch <= end_epoch:
        print("invalid epochs")
        return False
    
    if end_epoch - start_epoch < GOVERNANCE_GENESIS_PARAMETERS['min_proposal_voting_period']:
        print("invalid start epoch")
        return False
    
    if activation_epoch - end_epoch < GOVERNANCE_GENESIS_PARAMETERS['min_proposal_grace_epochs']:
        print("invalid activation epoch")
        return False
    
    if proposal_content > GOVERNANCE_GENESIS_PARAMETERS['max_proposal_content_size']:
        print("invalid content")
        return False
    
    if wasm_size > GOVERNANCE_GENESIS_PARAMETERS['max_proposal_code_size']:
        print("invalid wasm")
        return False
    
    return True



def build_proposal(
    content: Dict[str, str],
    author: str,
    start_epoch: int, 
    end_epoch: int, 
    activation_epoch: int, 
    data_path: str) -> str:
    return {
        "proposal": {
            "content": content,
            "author": author,
            "voting_start_epoch": start_epoch,
            "voting_end_epoch": end_epoch,
            "grace_epoch": activation_epoch
        },
        "data": list(open(data_path, "rb").read())
    }

def build_proposal_content(
        title: str,
        authors: str,
        discussions_to: str,
        licence: str,
        abstract: str,
        motivation: str,
        details: str,
        requires: str,
) -> Dict[str, str]:
    return {
        "title": title,
        "authors": authors,
        "discussions-to": discussions_to,
        "created": datetime.now().isoformat(),
        "license": licence,
        "abstract": abstract,
        "motivation": motivation,
        "details": details,
        "requires": requires
    }

def main(data_path: str, output_path: str):
    data = json.load(open(data_path, 'r'))

    validation_res = validate_data(data)

    if not validation_res:
        print("Invalid proposal data")
        exit(1)

    proposal_content = build_proposal_content(
        data["title"],
        data["authors"],
        data["discussions-to"],
        data["license"],
        data["abstract"],
        data["motivation"],
        data["details"],
        data["requires"]
    )

    proposal = build_proposal(
        proposal_content,
        data["author"],
        data["voting_start_epoch"],
        data["voting_end_epoch"],
        data["grace_epoch"],
        data["wasm_path"],
    )

    with open(output_path, 'w') as f:
        json.dump(proposal, f, ensure_ascii=False, indent=4)

  
if __name__=="__main__": 
    parser = argparse.ArgumentParser()
    parser.add_argument('-d', "--proposal-data-path", help="Path to the json file describing the proposal data", required=True)
    parser.add_argument('-o', "--output", help="Path to the generate proposal", required=True)
    
    args = parser.parse_args()

    main(args.proposal_data_path, args.output)