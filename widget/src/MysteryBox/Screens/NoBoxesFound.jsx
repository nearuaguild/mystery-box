const widget_owner_id = "nearukraineguild.near";

const font = fetch(
  "https://fonts.googleapis.com/css2?family=Kodchasan:wght@700&display=swap"
).body;

if (!font) {
  return <></>;
}

const Wrapper = styled.div`
  position: fixed;
  top: 64px;
  bottom: 0;
  left: 0;
  right: 0;
  width: 100%;
`;

const WrapperContent = styled.div`
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
`;

const Empty = styled.div``;

const WrapperText = styled.div`
  @media (min-width: 768px) {
    margin: 0em 4em;
  }
  @media (min-width: 512px) {
    margin: 0em 2em;
  }

  margin: 0em 1em;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: end;

  ${font}
`;

const Logo = styled.img`
  height: 100px;
`;

const PrimaryText = styled.div`
  @media (min-width: 512px) {
    font-size: 24px;
    line-height: 32px;
  }

  font-size: 18px;
  line-height: 24px;

  font-family: "Kodchasan", sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;
`;
const SecondaryText = styled.p`
  @media (min-width: 512px) {
    font-size: 24px;
    line-height: 32px;
  }

  font-size: 18px;
  line-height: 24px;

  font-family: "Kodchasan", sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #2bccc2;
  text-transform: uppercase;
`;

const WrapperSocial = styled.div`
  display: flex;
  margin-bottom: 2em;
  width: 180px;
  align-items: center;
  justify-content: space-around;
`;

const SocialText = styled.p`
  font-family: "Kodchasan", sans-serif;
  font-size: 12px;
  font-weight: 700;
  line-height: 16px;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;
  margin: 0;
`;

const SocialLink = styled.a`
  cursor: pointer;
  text-align: none;
  height: 16px;
  margin: 0;
  padding: 0;
`;

const SocialIcon = styled.img`
  height: 100%;
  vertical-align: unset;
`;

return (
  <Wrapper>
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Components.BackgroundStars`}
    />
    <WrapperContent>
      <Empty />
      <WrapperText>
        <Logo
          src="https://ipfs.near.social/ipfs/bafkreidsc7fcwi3urcpew2fiuvqw47d7i5bxydd64ttfqqp4f2m577khha"
          alt="Near Box logo"
        />
        <PrimaryText>
          Fortune has departed this time, but do not dismay!
        </PrimaryText>
        <PrimaryText>
          Join us next time and prepare for cosmic luck!
        </PrimaryText>
      </WrapperText>
      <WrapperSocial>
        <SocialText>Follow us</SocialText>
        <SocialLink href="https://twitter.com/nearuaguild" target="_blank">
          <SocialIcon
            src="https://ipfs.near.social/ipfs/bafkreibhvlipldq5qnolfb74ufbgqkbcwlim5vvtk3mbz6ujvbsar6fesq"
            alt="Twitter"
          />
        </SocialLink>
        <SocialLink href="https://t.me/nearprotocolua" target="_blank">
          <SocialIcon
            src="https://ipfs.near.social/ipfs/bafkreihcqu65spu6o5z6vw5atbjx7iqphzvlss3hvz4l7bj3syhvavzf5a"
            alt="Telegram"
          />
        </SocialLink>
        <SocialLink
          href="https://near.org/near/widget/ProfilePage?accountId=nearukraineguild.near"
          target="_blank"
        >
          <SocialIcon
            src="https://ipfs.near.social/ipfs/bafkreier4aong3uumu4ndl6iahol2kgeisfqtl6c237x3q34ql6smjvare"
            alt="Near Social"
          />
        </SocialLink>
      </WrapperSocial>
    </WrapperContent>
  </Wrapper>
);
