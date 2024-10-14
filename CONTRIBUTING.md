# snarkOS: Contribution Guide

This checklist provides a step-by-step guide for restarting the Aleo network with new features merged. Follow these steps to ensure a smooth process.

## Branches Overview

### `mainnet-staging` Branch
This branch serves as a staging area for the integration and initial testing of changes before they are promoted to the `mainnet` branch.

### `mainnet` Branch
The production branch where only stable and thoroughly tested changes are merged. It is used for creating production releases and is always a direct mirror of a `mainnet-staging` commit.

## Networks Overview

### DevNet(s)
Initial proposed changes are implemented and tested on DevNets. Snarkops aims to provide guides and scripts for managing SnarkOS and participating in ANF’s CanaryNet.

### CanaryNet (running mainnet-staging branch)
Changes are merged into CanaryNet from DevNet for testing and validation. CanaryNet is used to onboard additional validators before potentially bonding them to Testnet Beta/Mainnet. 
- **Tag Standard**: `canary-v*`. [Link to tags](https://github.com/AleoNet/snarkOS/tags).
- **Explorer for CanaryNet**: [Link to Explorer](https://vision.snarkos.net/?blocks).

### Testnet Beta (running mainnet branch)
An open, public network for testing applications in a production-like environment without incurring costs. Validators are bonded by the Aleo Network Foundation.
- **Tag Standard**: `testnet-beta-v*`. [Link to tags](#).
- **Explorer for Testnet Beta**: see link above

### Mainnet (running mainnet branch)
The final testing stage before full production deployment. Intended to be the “last stop” for new code and/or validator onboarding.
- **Tag Standard**: `mainnet-v*`.
- **Explorer Support**: There are several deployed blockchain explorers for Aleo. See:
    - [Provable Explorer](https://explorer.provable.com/)
    - [AleoScan](https://aleoscan.io)
    - [Aleo123](https://mainnet.aleo123.io/)

## Contribution Workflow

### 1. Fork the Repository
- Fork the repository from the `mainnet-staging` branch to your own GitHub account.
- Clone your fork locally:
  ```bash
    git clone git@github.com:AleoNet/snarkOS.git
    git remote add upstream git@github.com:AleoNet/snarkOS.git
  ```

### 2. Switch to the Base Branch
  ```bash
    git switch mainnet-staging
  ```

### 3. Create a Feature Branch
Create a feature branch from your fork’s main branch:
  ```bash
    git checkout -b feat/my-branch
  ```

### 4. Develop Your Feature/Fix and Test
- Develop your feature or fix in your forked repository.
- Run:
  ```bash
    ./snarkOS/devnet.sh
  ```
Make sure to approve the option to re-install the snarkOS binary to test with your current local snarkOS code. Verify that the network progresses normally and send some transactions for confirmation of network stability. Run any specific tests related to your feature/fix.

### 5. Push Your Code to Fix Branch
Commit changes with meaningful commit messages that clearly describe the changes and their purpose:
  ```bash
        git add .
        git commit -m "Add detailed description of the changes"
        git push
  ```

### 6. Submit a PR to Your Fork’s Main Branch
Submit a pull request (PR) from your feature branch to your fork’s main branch. This triggers the CI pipeline in your fork to run automated tests.

### 7. Internal Code Review
Request an internal code review from your team within your forked repository. Team members will review the code, suggest changes, and approve the PR if it meets quality standards. The CI pipeline is run again to ensure no new issues have been introduced during the review process.

### 8. Submit a PR to the Main AleoNet/snarkOS Repository
After internal approval, submit a PR from your fork’s main branch to the main AleoNet/snarkOS repository’s mainnet-staging branch. This triggers the main CI pipeline to run all relevant tests and checks again to ensure code stability and compatibility.

### 9. Review by Core Team

Core team members review the PR in the main AleoNet/snarkOS repository. If the CI pipeline passes and the review is successful, the PR is approved and will be included in the next release pending completion of the test plan

