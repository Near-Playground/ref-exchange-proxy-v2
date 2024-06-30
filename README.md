# Ref Exchange Proxy Contract

This project serves as a continuation of the previous project [Ref Proxy](https://github.com/Near-Playground/ref-proxy).

## Overview

As discussed in the previous project, we will rewrite the entire contract to improve its functionality and robustness. Our main objectives include:

1. **Error Handling:** Implement comprehensive error handling mechanisms to address issues such as slippage errors during swaps.
2. **FT Transfer Failures:** Resolve problems that arise from failures to transfer Fungible Tokens (FT), for instance, when the recipient has not registered a storage deposit on the FT contract.

## Objectives

### 1. Comprehensive Error Handling

We aim to implement mechanisms that handle any errors thrown during swaps, including but not limited to:

-   Slippage errors
-   Network issues
-   Contract execution failures

### 2. FT Transfer Failures

To address the failures in transferring FT, we will:

-   Ensure recipients have registered storage deposits on the FT contract
-   Implement fallback mechanisms for unregistered recipients

## Future Work

Beyond the immediate goals, we will explore additional improvements and optimizations to enhance the contract's efficiency and reliability.

For more information, refer to the original [Ref Proxy](https://github.com/Near-Playground/ref-proxy) project.
