const { href: linkHref } = VM.require('denbite.testnet/widget/core.lib.url');

linkHref || (linkHref = () => {});

const defaultContractExist =
  props.defaultContract && props.contracts.includes(props.defaultContract);
const defaultActiveIndex = defaultContractExist
  ? props.contracts.indexOf(props.defaultContract)
  : 0;

console.log('defaultActiveIndex', defaultActiveIndex);

State.init({
  active: defaultActiveIndex,
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

  flex-basis: 80%;

  height: 100%;
  padding: 8px;
`;

const MenuTitle = styled.p`
  font-family: 'Kodchasan', sans-serif;
  font-size: 24px;
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
  flex-basis: 25%;
`;
const MenuFooter = styled.div`
  display: flex;
  flex-direction: column;
  width: 100%;
  flex-basis: 45%;
`;
const MenuFooterRow = styled.div`
  display: flex;
  flex-grow: 1;
`;

const WrapperMenuButton = styled.div`
  flex-basis: 50%;

  display: flex;
  align-items: center;
  justify-content: center;
`;

const MenuButton = styled.button`
  height: 75%;
  width: 90%;

  border: 0;
  border-radius: 10px;

  background: #638caf;

  &:disabled {
    background: #334d64;

    a {
      color: #000000;
    }
  }
`;

const MenuLink = styled.a`
  height: 100%;
  width: 100%;

  display: flex;
  justify-content: center;
  align-items: center;

  font-family: 'Kodchasan', sans-serif;
  font-size: 13px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;

  color: #ffffff;

  &:hover {
    text-decoration: none;
  }
`;

console.log('render');

const previousActiveContract = () => {
  if (state.active === 0) return;

  State.update({ active: state.active - 1 });
};

const nextActiveContract = () => {
  if (state.active === props.contracts.length - 1) return;

  State.update({ active: state.active + 1 });
};

return (
  <>
    <Title>Contracts</Title>
    <SliderWrapper>
      <LeftArrow
        disabled={state.active === 0}
        onClick={previousActiveContract}
      />
      <WrapperMenu>
        <MenuHeader>
          <MenuTitle>Contract Name</MenuTitle>
          <MenuSubtitle>{props.contracts[state.active]}</MenuSubtitle>
        </MenuHeader>
        <MenuContent></MenuContent>
        <MenuFooter>
          <MenuFooterRow>
            <WrapperMenuButton>
              <MenuButton>
                <MenuLink
                  href={linkHref({
                    widgetSrc: 'denbite.testnet/widget/MysteryBox.Manage',
                    params: {
                      contract_id: props.contracts[state.active],
                      page: 'AddNearReward',
                    },
                  })}
                >
                  Add NEAR Reward
                </MenuLink>
              </MenuButton>
            </WrapperMenuButton>
            <WrapperMenuButton>
              <MenuButton>
                <MenuLink
                  href={linkHref({
                    widgetSrc: 'denbite.testnet/widget/MysteryBox.Manage',
                    params: {
                      contract_id: props.contracts[state.active],
                      page: 'AddNftReward',
                    },
                  })}
                >
                  Add NFT Reward
                </MenuLink>
              </MenuButton>
            </WrapperMenuButton>
          </MenuFooterRow>
          <MenuFooterRow>
            <WrapperMenuButton>
              <MenuButton>
                <MenuLink
                  href={linkHref({
                    widgetSrc: 'denbite.testnet/widget/MysteryBox.Manage',
                    params: {
                      contract_id: props.contracts[state.active],
                      page: 'MintBox',
                    },
                  })}
                >
                  Mint BOX
                </MenuLink>
              </MenuButton>
            </WrapperMenuButton>
            <WrapperMenuButton>
              <MenuButton>
                <MenuLink
                  href={linkHref({
                    widgetSrc: 'denbite.testnet/widget/MysteryBox.Manage',
                    params: {
                      contract_id: props.contracts[state.active],
                      page: 'ListRewards',
                    },
                  })}
                >
                  List Rewards
                </MenuLink>
              </MenuButton>
            </WrapperMenuButton>
          </MenuFooterRow>
          <MenuFooterRow>
            <WrapperMenuButton>
              <MenuButton>
                <MenuLink
                  href={linkHref({
                    widgetSrc: 'denbite.testnet/widget/MysteryBox.Manage',
                    params: {
                      contract_id: props.contracts[state.active],
                      page: 'ListUserBoxes',
                    },
                  })}
                >
                  List User Boxes
                </MenuLink>
              </MenuButton>
            </WrapperMenuButton>
            <WrapperMenuButton>
              <MenuButton disabled>
                <MenuLink>Statistics</MenuLink>
              </MenuButton>
            </WrapperMenuButton>
          </MenuFooterRow>
        </MenuFooter>
      </WrapperMenu>
      <RightArrow
        disabled={state.active === props.contracts.length - 1}
        onClick={nextActiveContract}
      />
    </SliderWrapper>
    <Widget
      src={`denbite.testnet/widget/MysteryBox.Manage.Components.SubmitButton`}
      props={{
        text: 'Create new contract',
        onClick: () => {},
      }}
    />
  </>
);
