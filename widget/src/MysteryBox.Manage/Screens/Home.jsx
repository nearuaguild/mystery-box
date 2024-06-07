const widget_owner_id = "untidy-scarecrow.testnet";

const { href: linkHref } = VM.require(`${widget_owner_id}/widget/core.lib.url`);

linkHref || (linkHref = () => {});

const defaultContractIndex = props.defaultContractId
  ? (props.contracts || []).findIndex(
      (contract) => contract.contract_id === props.defaultContractId
    )
  : -1;
const defaultActiveIndex =
  defaultContractIndex !== -1 ? defaultContractIndex : 0;

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

const WrapperMenu = styled.div`
  background: rgba(24, 36, 50, 1);
  border: 0;
  border-radius: 30px;

  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;

  width: 80%;

  height: 100%;
  padding: 8px;
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

const Bottom = styled.div`
  display: flex;
  flex-direction: row;

  justify-content: space-between;
  align-items: center;

  width: 500px;

  @media (max-width: 678px) {
    flex-direction: column;

    height: 84px;

    justify-content: space-around;
  }
`;

const previousActiveContract = () => {
  if (state.active === 0) return;

  State.update({ active: state.active - 1 });
};

const nextActiveContract = () => {
  if (state.active === props.contracts.length - 1) return;

  State.update({ active: state.active + 1 });
};

const contract = props.contracts[state.active];

const createLinkToPage = (page) => {
  return linkHref({
    widgetSrc: `${widget_owner_id}/widget/MysteryBox.Manage`,
    params: {
      contract_id: contract.contract_id,
      page,
    },
  });
};

return (
  <>
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.Title`}
      props={{
        text: 'Contracts ololo',
      }}
    />
    <SliderWrapper>
      <LeftArrow
        disabled={state.active === 0}
        onClick={previousActiveContract}
      />
      <WrapperMenu>
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.MenuHeader`}
          props={{
            title: contract.title,
            subtitle: contract.contract_id,
          }}
        />
        <MenuContent></MenuContent>
        <MenuFooter>
          <MenuFooterRow>
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.MenuButton`}
              props={{
                text: 'Add NEAR reward',
                href: createLinkToPage('AddNearReward'),
              }}
            />
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.MenuButton`}
              props={{
                text: 'Add NFT reward',
                href: createLinkToPage('AddNftReward'),
              }}
            />
          </MenuFooterRow>
          <MenuFooterRow>
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.MenuButton`}
              props={{
                text: 'Mint BOX',
                href: createLinkToPage('MintBox'),
              }}
            />
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.MenuButton`}
              props={{
                text: 'List Rewards',
                href: createLinkToPage('ListRewards'),
              }}
            />
          </MenuFooterRow>
          <MenuFooterRow>
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.MenuButton`}
              props={{
                text: 'List User Boxes',
                href: createLinkToPage('ListUserBoxes'),
              }}
            />
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.MenuButton`}
              props={{
                disabled: true,
                text: 'Statistics',
              }}
            />
          </MenuFooterRow>
        </MenuFooter>
      </WrapperMenu>
      <RightArrow
        disabled={state.active === props.contracts.length - 1}
        onClick={nextActiveContract}
      />
    </SliderWrapper>
    <Bottom>
      <Widget
        src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryLinkButton`}
        props={{
          text: 'Create another contract',
          href: linkHref({
            widgetSrc: `${widget_owner_id}/widget/MysteryBox.Manage`,
            params: {
              page: 'DeployContract',
            },
          }),
        }}
      />
      <Widget
        src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryLinkButton`}
        props={{
          text: 'View Claiming Page',
          href: linkHref({
            widgetSrc: `${widget_owner_id}/widget/MysteryBox`,
            params: {
              contract_id: contract.contract_id,
            },
          }),
          target: '_blank',
        }}
      />
    </Bottom>
  </>
);
