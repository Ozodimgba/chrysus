# Chrysus Protocol
> Bitcoin-restaking for SOON rollup security leveraging Zeus Network's permissionless infrastructure

## Overview

Chrysus Protocol enables Bitcoin holders to provide economic security for SOON rollups through Zeus Network's permissionless infrastructure. By leveraging APOLLO's zBTC system and the Guardian network, Chrysus creates a trustless security layer backed by Bitcoin's economic value.

## Architecture

### Zeus Network Integration

Chrysus is built on three core Zeus Network components:

1. **APOLLO Integration**
   - Uses APOLLO's zBTC as the base security token
   - Permissionless conversion of BTC to zBTC
   - Managed through Zeus Program Library (ZPL)

2. **Guardian Network**
   - Decentralized verification of Bitcoin transactions
   - Multi-Guardian signature requirements
   - Trustless security validation

3. **Zeus Program Library (ZPL)**
   - Cross-chain communication infrastructure
   - State management and verification
   - Security parameter enforcement

## How It Works

### BTC to Security Flow

1. **Initial BTC Lock**
   - User sends BTC to Zeus Network address
   - Guardian network validates Bitcoin transaction
   - Multiple Guardians must sign to confirm
   - APOLLO mints equivalent zBTC

2. **Security Position Creation**
   - zBTC gets locked in Chrysus contract
   - Security parameters are established
   - Validator rights are assigned

3. **L2 Validation**
   - Validators operate L2 with zBTC as collateral
   - Performance monitoring through Guardian network
   - Automated slashing for misbehavior

### Security Model

#### Guardian Verification
- Decentralized network of ZeusNodes
- Multiple signatures required for actions
- Real-time monitoring of validator behavior
- Automated response to security violations

#### zBTC Collateral
- Value backed by actual Bitcoin
- Permissionless conversion through APOLLO
- Slashing enforced through Guardian consensus
- Automated penalty execution

## Integration Guide

### For Validators

1. **Setup Requirements**
   - Minimum BTC amount for validation
   - Zeus Network account
   - L2 operator credentials

2. **Getting Started**
   - Convert BTC to zBTC through APOLLO
   - Register as Chrysus validator
   - Set up L2 validation node
   - Configure monitoring systems

### For L2 Networks

1. **Integration Requirements**
   - Zeus Network compatibility
   - Guardian network integration
   - APOLLO contract interface

2. **Security Setup**
   - Define slashing parameters
   - Set minimum stake requirements
   - Configure Guardian verification thresholds

## Security Considerations

### Guardian Network
- Multi-signature requirements
- Decentralized verification
- Byzantine fault tolerance
- Real-time monitoring

### Bitcoin Security
- Block confirmation requirements
- Transaction verification process
- Guardian consensus thresholds

### L2 Security
- Slashing conditions
- Dispute resolution


## Future Development

### Phase 1: Core Implementation
- Basic security staking
- Guardian integration
- APOLLO interface

### Phase 2: Enhanced Features
- Advanced slashing mechanics
- Multi-L2 support
- Performance optimization

### Phase 3: Ecosystem Expansion
- Additional L2 integrations
- Enhanced monitoring tools
- Advanced security features

## Getting Started

### Prerequisites
- Bitcoin wallet
- Zeus Network account
- L2 operator status

### Quick Start
1. Deposit BTC through Zeus Network
2. Convert to zBTC via APOLLO
3. Register as validator
4. Configure L2 operation

## License
MIT License
