use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface TokenData {
        function name() external view returns (string memory);

        function symbol() external view returns (string memory);

        function decimals() external view returns (uint8);

        function totalSupply() external view returns (uint256);

        function contractURI() external view returns (string memory);
    }
);
