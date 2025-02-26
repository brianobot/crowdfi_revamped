# Crowdfi - Solana Smart Contract

## Overview
Crowdfi is a Solana-based smart contract that enables users to create crowdfunding campaigns for sponsorship. The platform ensures transparency and trust by guaranteeing that donations are refundable if the campaign does not meet its target amount within the specified duration. This README provides an overview of the project, its features, and instructions for setting up and interacting with the smart contract.

## Features
- Campaign Creation: Users can create crowdfunding campaigns by specifying a target amount, duration, and other details.

- Donations: Supporters can contribute SOL to campaigns they wish to sponsor.

- Refund Mechanism: If a campaign does not meet its target amount within the specified duration, donors can claim a refund of their contributions.

- Transparency: All campaign details and transactions are recorded on the Solana blockchain, ensuring transparency and immutability.

- Withdrawal: Campaign creators can withdraw funds only if the target amount is met within the campaign duration.


## Smart Contract Functions
### For Campaign Creators
1. create_campaign: Allows users to create a new campaign by specifying:

    - Target amount (in SOL)
    - Campaign duration (in seconds)
    - Campaign title and description

2. withdraw_funds: Enables the campaign creator to withdraw funds if the campaign is successful (i.e., the target amount is met within the duration).

### For Donors
1. donate: Allows users to donate SOL to a specific campaign.
2. claim_refund: Enables donors to claim a refund if the campaign does not meet its target amount within the specified duration.

### For Everyone
1. get_campaign_details: Retrieves details about a specific campaign, including:

- Target amount
- Amount raised
- Duration
- Status (active/ended)
- List of donors

## Setup and Deployment
Prerequisites
- Solana CLI installed (Installation Guide)
- Rust and Cargo installed (Installation Guide)
- Anchor framework installed (Installation Guide)

Steps to Deploy
Clone the repository:

```bash
git clone https://github.com/your-repo/crowdfi.git
```

```bash
cd crowdfi
```
Build the smart contract:

```bash
anchor build
```
Deploy the contract to the Solana network:

```bash
anchor deploy
```
Update the program ID in the Anchor.toml file and the lib.rs file with the deployed program ID.

Testing
Run the test suite to ensure the smart contract functions as expected:

```bash
anchor test
```

## Interacting with the Contract

### Using a Frontend
You can build a frontend application using frameworks like React or Vue.js to interact with the smart contract. Use the @solana/web3.js library to connect to the Solana blockchain and call the smart contract functions.

## Security Considerations
- Ensure that the campaign duration and target amount are validated to prevent malicious campaigns.

- Implement proper access control to restrict fund withdrawals to only the campaign creator.

- Test the refund mechanism thoroughly to ensure donors can reclaim their funds if the campaign fails.

## Testing

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Contact
For questions or support, reach out to the maintainers:

Email: brianobot9@gmail.com <br>
Twitter: [Brian Obot](https://x.com/i_am_brian_obot)

Thank you for using Crowdfi! We hope this platform empowers creators and sponsors to collaborate transparently and effectively. 🚀