const font = fetch(
  'https://fonts.googleapis.com/css2?family=Kodchasan:wght@300;400;500;700&display=swap'
).body;

if (!font) {
  return <></>;
}

const Container = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  align-items: center;

  position: fixed;
  top: 64px;
  bottom: 0;
  left: 0;
  right: 0;
  width: 100%;

  @media (max-width: 991px) {
    top: 72px;
  }

  background: radial-gradient(50% 50% at 50% 50%, #5e85a6 0%, #496b88 100%);
  background-repeat: no-repeat;
  background-position: center;
  background-size: cover;

  ${font}
`;

const Header = styled.div`
  max-height: 64px;
  flex-grow: 1;

  @media (max-width: 512px) {
    max-height: 48px;
  }

  width: 96%;

  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
`;

const Empty = styled.div`
  flex: 1;
`;

const Content = styled.div`
  flex-grow: 1;

  width: 100%;

  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-evenly;
`;

const Footer = styled.div`
  max-height: 54px;
  flex-grow: 1;

  display: flex;
  align-items: center;
  justify-content: center;
`;

const Logo = styled.img`
  height: 72%;
`;

const HeaderLogo = styled.div`
  flex: 1;

  height: 100%;

  display: flex;
  align-items: center;
  justify-content: flex-start;
`;

const HeaderButton = styled.div`
  flex: 1;

  height: 100%;

  display: flex;
  align-items: center;
  justify-content: center;
`;

function Layout({ children, contract_id, active_home_button }) {
  return (
    <Container>
      <Header>
        <HeaderLogo>
          <Logo
            src="https://ipfs.near.social/ipfs/bafkreicrgaxj2jf7wxdiddoqowdnajpixa7vjusr2pi6oy2khitslxsjiu"
            alt="Mystery Box Logo"
          />
        </HeaderLogo>
        <HeaderButton>
          <Widget
            src="denbite.testnet/widget/MysteryBox.Manage.Components.HomeButton"
            props={{
              contract_id: contract_id,
              active: active_home_button,
            }}
          />
        </HeaderButton>
        <Empty />
      </Header>
      <Content>{children}</Content>
      <Footer>
        <Widget src="denbite.testnet/widget/MysteryBox.Manage.Components.Socials" />
      </Footer>
    </Container>
  );
}

return { Layout };
