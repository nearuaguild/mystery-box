const widget_owner_id = "untidy-scarecrow.testnet";

console.log('props', props);

const tokens = props.tokens || [];

State.init({
  active: 0,
  rarities: tokens.map((token) => ({
    contract: token.contract,
    token_id: token.token.token_id,
    value: '',
  })),
});

const SliderWrapper = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;

  flex-basis: 70%;

  width: 48%;

  @media (max-width: 1024px) {
    width: 60%;
  }

  @media (max-width: 786px) {
    width: 75%;
  }
  @media (max-width: 512px) {
    width: 90%;
  }
`;

const Svg = styled.svg`
  height: 36px;
  cursor: pointer;

  ${(props) =>
    !props.disabled
      ? `filter: drop-shadow(0px 0px 4px rgba(43, 204, 193, 0.5));`
      : `filter: none;`}
`;

const RightArrow = ({ onClick, disabled }) => (
  <Svg
    viewBox="0 0 35 58"
    disabled={disabled}
    onClick={onClick}
    xmlns="http://www.w3.org/2000/svg"
  >
    <g xmlns="http://www.w3.org/2000/svg">
      <path
        fill-rule="evenodd"
        clip-rule="evenodd"
        d="M0.550369 4.94975L5.50011 0L29.521 24.0209L29.542 24L34.4917 28.9497L34.4708 28.9707L34.4915 28.9914L29.5417 33.9411L29.521 33.9204L5.50032 57.9411L0.550575 52.9914L24.5713 28.9707L0.550369 4.94975Z"
        fill={disabled ? '#818B94' : '#fff'}
      />
    </g>
  </Svg>
);

const LeftArrow = ({ onClick, disabled }) => (
  <Svg
    viewBox="0 0 35 58"
    xmlns="http://www.w3.org/2000/svg"
    disabled={disabled}
    onClick={onClick}
  >
    <g xmlns="http://www.w3.org/2000/svg" transform="matrix(-1 0 0 -1 35 58)">
      <path
        fill-rule="evenodd"
        clip-rule="evenodd"
        d="M0.550369 4.94975L5.50011 0L29.521 24.0209L29.542 24L34.4917 28.9497L34.4708 28.9707L34.4915 28.9914L29.5417 33.9411L29.521 33.9204L5.50032 57.9411L0.550575 52.9914L24.5713 28.9707L0.550369 4.94975Z"
        fill={disabled ? '#818B94' : '#fff'}
      />
    </g>
  </Svg>
);

const WrapperLogo = styled.div`
  position: absolute;
  top: 24px;
  left: 24px;
`;

const Logo = styled.img`
  height: 50px;

  @media (max-width: 768px) {
    height: 42px;
  }

  @media (max-width: 512px) {
    height: 32px;
  }
`;

const HomeButton = styled.button`
  border: 0;
  background: none;
`;

const Title = styled.p`
  font-family: 'Kodchasan', sans-serif;
  font-size: 28px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;
  margin: 0;
`;

const WrapperMenu = styled.div`
  background: rgba(24, 36, 50, 1);
  border: 0;
  border-radius: 30px;

  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;

  flex-basis: 84%;

  height: 100%;
`;

const MenuTitle = styled.p`
  font-family: 'Kodchasan', sans-serif;
  font-size: 28px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  margin: 0;
`;

const MenuSubtitle = styled.p`
  font-family: 'Kodchasan', sans-serif;
  font-size: 14px;
  font-weight: 400;
  letter-spacing: 0em;
  text-align: center;
  color: rgba(43, 204, 194, 1);
  margin: 0;
`;

const MenuHeader = styled.div`
  flex-basis: 20%;
`;

const MenuContent = styled.div`
  display: flex;
  width: 100%;
  flex-basis: 55%;

  justify-content: center;
  align-items: center;
`;

const WrapperNftPreview = styled.div`
  width: 80%;
  height: 100%;

  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-direction: column;

  border: 0;
  border-radius: 30px;

  ${(props) => {
    if (!props.url) return `background: #638caf;`;

    return `
    background-image: url(${props.url});
    background-size: cover;
    background-position: center;
    `;
  }}

  box-shadow: 0px 40px 40px 0px #00000033;
  padding-top: 4px;
  padding-bottom: 4px;
`;

const NftTokenTitle = styled.div`
  width: 90%;
  flex-basis: 32px;

  border-radius: 30px;
  background: #182432;

  display: flex;
  justify-content: center;
  align-items: center;

  margin: 0;

  font-family: Kodchasan;
  font-size: 16px;
  font-weight: 500;
  text-align: center;

  color: white;
`;

const NftCollection = styled.div`
  width: 90%;
  flex-basis: 32px;

  display: flex;
  justify-content: flex-start;
  align-items: center;

  margin: 0;

  font-family: Kodchasan;
  font-size: 11px;
  font-weight: 300;
  text-align: center;

  color: white;
`;

const NftCollectionLogo = styled.img`
  width: 32px;
  height: 32px;

  border-radius: 50%;
  background: #182432;
`;

const NftCollectionTitle = styled.p`
  font-family: Kodchasan;
  font-size: 14px;
  font-weight: 500;
  text-align: center;

  color: white;

  margin: 0;
  margin-left: 8px;
`;

const MenuFooter = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: 100%;
  flex-basis: 15%;
`;

const RarityPicker = styled.div`
  width: 80%;
  height: 40px;

  border: 0;
  border-radius: 10px;
  background: #202f3f;

  display: flex;
  justify-content: space-evenly;
  align-items: center;
`;

const RarityText = styled.p`
  margin: 0;

  font-family: Kodchasan;
  font-size: 11px;
  font-weight: 300;
  text-align: center;

  color: white;
`;

const RarityPickerMenu = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;

  flex-basis: 100px;
`;

const rarities = state.rarities || [];

console.log('rarities', rarities);

const previousActiveToken = () => {
  if (state.active === 0) return;

  State.update({ active: state.active - 1 });
};

const nextActiveToken = () => {
  if (state.active === tokens.length - 1) return;

  State.update({ active: state.active + 1 });
};

const token = tokens[state.active];
const rarity = rarities[state.active]?.value;

const getMediaUrlForToken = (nftToken) => {
  const { token, metadata } = nftToken;

  if (!metadata || !token) return null;

  if (!token.metadata?.media) return undefined;

  if (!metadata.base_uri) return token.metadata?.media;

  const url = metadata.base_uri + '/' + token.metadata.media;

  return url;
};

const getTitleForToken = (nftToken) => {
  const { token } = nftToken;

  return token.metadata.title;
};

const getCollectionTitleForToken = (nftToken) => {
  const { metadata } = nftToken;

  return metadata.name;
};

const getCollectionIconForToken = (nftToken) => {
  const { metadata } = nftToken;

  return metadata.icon;
};

const updateRarity = (contract, token_id, value) => {
  State.update((previousState) => {
    const { rarities } = previousState;

    const rarity = rarities.find(
      (rarity) => rarity.contract === contract && rarity.token_id === token_id
    );

    if (rarity === -1) return previousState;

    rarity.value = value;

    return {
      ...previousState,
      rarities,
    };
  });
};

const someRarityIsChosen = rarities.some((rarity) => !!rarity.value);

const shouldSubmitButtonBeDisabled = !someRarityIsChosen;

const submitTransactionToAddNftRewards = () => {
  console.log('submitTransactionToAddNftRewards', rarities);

  if (!someRarityIsChosen) return;

  const txn = rarities
    .filter((rarity) => !!rarity.value)
    .map((rarity) => {
      return {
        contractName: rarity.contract,
        methodName: 'nft_transfer_call',
        args: {
          receiver_id: props.contract?.contract_id,
          token_id: rarity.token_id,
          msg: rarity.value,
        },
        gas: Big(10).pow(12).mul(50), // 50 TGas
        deposit: 1, // 0.00064N + accounts
      };
    });

  Near.call(txn);
};

return (
  <>
    <Title>Add NFT Reward</Title>
    <SliderWrapper>
      <LeftArrow disabled={state.active === 0} onClick={previousActiveToken} />
      <WrapperMenu>
        <MenuHeader>
          <MenuTitle>{props.contract?.title}</MenuTitle>
          <MenuSubtitle>{props.contract?.contract_id}</MenuSubtitle>
        </MenuHeader>
        <MenuContent>
          <WrapperNftPreview url={getMediaUrlForToken(token)}>
            <NftTokenTitle>{getTitleForToken(token)}</NftTokenTitle>
            <NftCollection>
              <NftCollectionLogo src={getCollectionIconForToken(token)} />
              <NftCollectionTitle>
                {getCollectionTitleForToken(token)}
              </NftCollectionTitle>
            </NftCollection>
          </WrapperNftPreview>
        </MenuContent>
        <MenuFooter>
          <RarityPicker>
            <Svg
              width={24}
              height={24}
              viewBox="0 0 26 26"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              disabled={!rarity}
              onClick={() => {
                updateRarity(token.contract, token.token.token_id, '');
              }}
            >
              <path
                d="M22.192 3.808A12.915 12.915 0 0 0 13 0a12.915 12.915 0 0 0-9.192 3.808A12.915 12.915 0 0 0 0 13c0 3.472 1.352 6.737 3.808 9.192A12.915 12.915 0 0 0 13 26c3.472 0 6.737-1.352 9.192-3.808A12.915 12.915 0 0 0 26 13c0-3.472-1.352-6.737-3.808-9.192ZM13 24.477C6.672 24.477 1.523 19.328 1.523 13S6.672 1.523 13 1.523 24.477 6.672 24.477 13 19.328 24.477 13 24.477Z"
                fill="#8DBFEA"
              />
              <path
                d="M14.537 9.991h-4.72l.811-.727a.688.688 0 0 0 0-1.047.893.893 0 0 0-1.167 0l-2.22 1.991a.688.688 0 0 0 0 1.047l2.22 1.992a.873.873 0 0 0 .583.216.873.873 0 0 0 .584-.216.688.688 0 0 0 0-1.048l-.81-.727h4.72c1.55 0 2.811 1.132 2.811 2.524 0 1.39-1.261 2.523-2.812 2.523h-2.886c-.456 0-.825.332-.825.74 0 .41.369.741.825.741h2.886C16.998 18 19 16.204 19 13.996c0-2.209-2.002-4.005-4.463-4.005Z"
                fill="#8DBFEA"
              />
            </Svg>
            <RarityText>Set NFT Rarity</RarityText>
            <RarityPickerMenu>
              <Widget
                src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.RarityButton`}
                props={{
                  rarity: 'rare',
                  active: rarity === 'rare',
                  onClick: () => {
                    updateRarity(token.contract, token.token.token_id, 'rare');
                  },
                  tooltip: 'Rare',
                }}
              />
              <Widget
                src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.RarityButton`}
                props={{
                  rarity: 'epic',
                  active: rarity === 'epic',
                  onClick: () => {
                    updateRarity(token.contract, token.token.token_id, 'epic');
                  },
                  tooltip: 'Epic',
                }}
              />
              <Widget
                src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.RarityButton`}
                props={{
                  rarity: 'legendary',
                  active: rarity === 'legendary',
                  onClick: () => {
                    updateRarity(
                      token.contract,
                      token.token.token_id,
                      'legendary'
                    );
                  },
                  tooltip: 'Legendary',
                }}
              />
            </RarityPickerMenu>
          </RarityPicker>
        </MenuFooter>
      </WrapperMenu>
      <RightArrow
        disabled={state.active === tokens.length - 1}
        onClick={nextActiveToken}
      />
    </SliderWrapper>
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.SubmitButton`}
      props={{
        text: 'Submit',
        disabled: shouldSubmitButtonBeDisabled,
        onClick: submitTransactionToAddNftRewards,
      }}
    />
  </>
);
