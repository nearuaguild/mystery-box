const contract_id = props.contract_id;
const account_id = "helpua.testnet";
// const account_id = context.accountId;

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: start;
`;

const TextMessage = styled.p`
  font-size: 24px;
  color: #333;
`;

const CardListWrapper = styled.div`
  display: flex;
  flex-wrap: wrap;
  justify-content: space-evenly;
  align-items: center;
  width: 100%;
`;

const Card = styled.div`
  width: calc(25% - 25px); /* 25% for 4 cards in a row with spacing */

  @media (max-width: 848px) {
    width: calc(33% - 25px); /* 33% for 3 cards in a row with spacing */
  }

  @media (max-width: 624px) {
    width: calc(50% - 20px); /* 50% for 2 cards in a row with spacing */
  }

  @media (max-width: 420px) {
    width: 100%; /* 100% for 1 card in a row */
    padding: 20px 40px;
  }

  margin: 10px;
  padding: 20px 20px;
  border: 0.5px solid #ccc;
  border-radius: 2px;
  box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  align-items: center;
`;

const CardImage = styled.img`
  width: 100%;
  max-height: 250px;
  object-fit: cover;
  border-radius: 10%;
`;

const Divider = styled.div`
  width: 100%;
  height: 1px;
  background-color: #ccc;
  margin: 10px 0;
`;

const CardTitle = styled.h2`
  font-size: 16px;
  margin: 10px 0;
`;

const CardText = styled.p`
  font-size: 14px;
  color: #b50000;
`;

const CardButton = styled.button`
  background: white;
  font-size: 1.25em;
  padding: 0.25em 1em;
  border: 2px solid #f1c40f88;
  border-radius: 10px;
`;

console.log("contract", contract_id);
console.log("account", account_id);

const tokens = Near.view(contract_id, "nft_tokens_for_owner", {
  account_id: account_id,
});

const base_ipfs = "https://ipfs.near.social/ipfs/";

console.log("tokens", tokens);

if (tokens == null) return <></>;

if (tokens.length === 0) {
  return (
    <Wrapper>
      <TextMessage>You don't have any boxes</TextMessage>
    </Wrapper>
  );
}

const claim = (id) => {
  Near.call(contract_id, "claim_reward", {
    token_id: id,
  });
};

const CardListComponent = ({ tokens }) => {
  return (
    <CardListWrapper>
      {tokens.map((token, index) => {
        const url =
          base_ipfs +
          "bafkreifnb4tjmrocu2ntlqdbz2k76wfbmwaossbmxbsipxn2mdkum5fpga";
        // const url = base_ipfs + token.metadata.media;

        const isClaimed = Near.view(contract_id, "nft_is_claimed", {
          token_id: token.token_id,
        });

        return (
          <Card key={index}>
            <CardImage src={url} alt={"Mystery Box image"} />
            <CardTitle>{token.metadata.title}</CardTitle>
            <CardTitle>Rarity: rare</CardTitle>
            {isClaimed === false && (
              <CardTitle>Possible rewards: 2N (60%), NEARNauts NFT (5%), 5N (15%), 3N (20%)</CardTitle>
            )}
            <Divider />
            {isClaimed === true && <CardText>Already claimed - 5N</CardText>}
            {isClaimed === false && (
              <CardButton onClick={() => claim(token.token_id)}>
                Claim
              </CardButton>
            )}
          </Card>
        );
      })}
    </CardListWrapper>
  );
};

return (
  <Wrapper>
    <TextMessage>Your rewards</TextMessage>
    <CardListComponent tokens={tokens} />
  </Wrapper>
);
