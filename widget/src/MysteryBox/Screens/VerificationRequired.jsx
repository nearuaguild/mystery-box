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
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
  background: linear-gradient(rgba(17, 22, 31, 0.5), rgba(17, 22, 31, 0.5)),
    url("https://ipfs.near.social/ipfs/bafkreiglg2ftzn7vtylgnq5uq2dhozyiynulqttburfoeowl2ce7i5xfne");
  background-repeat: no-repeat;
  background-position: center;
  background-size: cover;
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
  margin: 0em 1em;
  height: ${props.height}px;
`;

const PrimaryText = styled.div`
  @media (min-width: 512px) {
    font-size: 26px;
    line-height: 34px;
  }

  margin-top: 1em;

  font-size: 20px;
  line-height: 27px;

  font-family: "Kodchasan", sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
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

const ButtonLink = styled.a`
  position: relative;
  margin-top: 1em;

  cursor: pointer;
  text-align: none;
  padding: 0.5em 2em;
  text-decoration: none;

  @media (min-width: 512px) {
    font-size: 26px;
    line-height: 34px;
  }

  font-size: 20px;
  line-height: 27px;
  font-family: "Kodchasan", sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;

  &:hover {
    text-decoration: none;
  }

  &:after {
    content: "";
    position: absolute;
    inset: 0;
    border-radius: 100px;
    border: 3px solid transparent;
    background: linear-gradient(
        92.13deg,
        #d2c659 -11.04%,
        #cb84c3 40.76%,
        #5c91df 101.98%
      )
      border-box;
    -webkit-mask: /*4*/ linear-gradient(#fff 0 0) padding-box,
      linear-gradient(#fff 0 0);
    -webkit-mask-composite: xor; /*5'*/
    mask-composite: exclude; /*5*/
    box-shadow: 0px 8px 24px rgba(21.48, 26.91, 35.06, 0.25);
  }
`;

return (
  <Wrapper>
    <WrapperContent>
      <Empty />
      <WrapperText>
        <div>
          <Logo
            src="https://ipfs.near.social/ipfs/bafkreibkxmrszgcyujkinannlt6lznfxar3x6yqnmwb4vvcllqrac2k2oa"
            alt="I-Am-Human logo"
            height={75}
          />
          <Logo
            src="https://ipfs.near.social/ipfs/bafkreifo7hbzcaicnsg4zfgdt7sve7ckzny5chgbtxuel72daqdrhabiuu"
            alt="Near Box logo"
            height={65}
          />
        </div>
        <PrimaryText>To proceed, make sure you've done KYC</PrimaryText>
        {/* @todo: add blur effect */}
        <ButtonLink href={props.url} target="_blank">
          Get verified
        </ButtonLink>
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
