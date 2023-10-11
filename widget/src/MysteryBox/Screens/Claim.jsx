console.log("Claim.props", props);

const base_ipfs = "https://ipfs.near.social/ipfs/";

State.init({
  active: 0,
});

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

  ${font}
`;

const WrapperContent = styled.div`
  background: radial-gradient(50% 50% at 50% 50%, #203343 0%, #0e121e 100%);
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;

  overflow-y: scroll;
  overflow-x: hidden;
`;

const SliderWrapper = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;

  min-height: 60%;
`;

const LeftArrow = styled.div`
  ackground: ${(props) => (props.$primary ? "#BF4F74" : "white")};

  transform: rotate(135deg);
  cursor: pointer;
  width: 24px;
  height: 24px;
  border-left: none;
  border-top: none;
  border-right: 4px solid;
  border-bottom: 4px solid;
  border-color: ${(props) => (props.disabled ? "gray" : "white")};
  border-radius: 1px;
  box-shadow: 3px 3px 3px rgba(15, 15, 15, 0.45);
`;

const RightArrow = styled.div`
  transform: rotate(-45deg);
  cursor: pointer;
  width: 24px;
  height: 24px;
  border-left: none;
  border-top: none;
  border-right: 4px solid;
  border-bottom: 4px solid;

  border-color: ${(props) => (props.disabled ? "gray" : "white")};
  border-radius: 1px;
  box-shadow: 3px 3px 3px rgba(15, 15, 15, 0.45);
`;

const SingleBoxWrapper = styled.div`
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
  height: 100%;

  display: ${(props) => (props.active ? "flex" : "none")};
`;

const levitation = styled.keyframes`
  from {
    transform: translateY(-12px) rotate(-3deg);
  }
  to {
    transform: translateY(12px) rotate(3deg);
  }
`;

const BoxImage = styled.img`
  object-fit: cover;
  max-width: 200px;

  animation: ${levitation} 3s infinite alternate-reverse;
`;

const BoxTitle = styled.div`
  background: rgba(14, 18, 30, 0.5);
  border-radius: 50px;
  padding: 0.25em 3em;
  display: flex;
  justify-content: center;
  align-items: center;

  font-size: 14px;
  line-height: 20px;
  font-family: "Kodchasan", sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;
`;

const BoxRewardWrapper = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
`;

const BoxRewardAmounts = styled.div`
  display: flex;
  flex-direction: column;
  align-items: end;
  justify-content: space-between;
  margin: 0;
  margin-right: 0.5em;
  padding: 0;
`;

const BoxRewardAmount = styled.p`
  color: rgba(161, 224, 234, 1);
  font-family: "Kodchasan", sans-serif;
  font-size: 12px;
  font-weight: 700;
  line-height: 18px;
  letter-spacing: 0em;
  text-align: center;
  text-transform: uppercase;
  margin: 0;
  padding: 0;
`;

const BoxRewardTitles = styled.div`
  display: flex;
  flex-direction: column;
  align-items: start;
  justify-content: space-between;
  margin: 0;
  padding: 0;
  margin-left: 0.5em;
`;

const BoxRewardTitle = styled.p`
  color: rgba(255, 255, 255, 1);
  font-family: "Kodchasan", sans-serif;
  font-size: 12px;
  font-weight: 700;
  line-height: 18px;
  letter-spacing: 0em;
  text-align: center;
  text-transform: uppercase;
  margin: 0;
  padding: 0;
`;

const ClaimButton = styled.div`
  position: relative;

  cursor: pointer;
  text-align: none;
  text-decoration: none;

  @media (min-width: 512px) {
    font-size: 20px;
    padding: 0.65em 2em;
  }

  font-size: 16px;
  line-height: 1;
  padding: 0.6875em 2.5em;

  font-family: "Kodchasan", sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;

  background: none;

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
    // box-shadow: 0px 8px 24px rgba(21.48, 26.91, 35.06, 0.25);
  }
`;

const OpenedBoxRewardTitle = styled.p`
  color: rgba(254, 185, 3, 1);
  font-family: "Kodchasan", sans-serif;
  font-size: 16px;
  font-weight: 700;
  line-height: 22px;
  letter-spacing: 0em;
  text-align: center;
  text-transform: uppercase;
  margin: 0;
  padding: 0;
`;

const ClaimedButton = styled.div`
  position: relative;

  cursor: default;

  text-align: none;
  text-decoration: none;

  @media (min-width: 512px) {
    font-size: 20px;
  }

  font-size: 16px;
  line-height: 1;
  padding: 0.5em 2em;

  font-family: "Kodchasan", sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;

  background: none;
  border-radius: 100px;
  border: 3px solid rgb(43, 204, 194);
  box-shadow: 0 0 25px rgba(43, 204, 194, 0.25),
    inset 0 0 25px rgba(43, 204, 194, 0.25);
`;

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
`;

const Logo = styled.img`
  height: 55px;
`;

const PrimaryText = styled.div`
  @media (min-width: 512px) {
    font-size: 24px;
    line-height: 30px;
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

const WrapperSocial = styled.div`
  display: flex;
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

const previousActiveBox = () => {
  if (state.active === 0) return;

  State.update({ active: state.active - 1 });
};

const nextActiveBox = () => {
  if (state.active === boxes.length - 1) return;

  State.update({ active: state.active + 1 });
};

const ClosedBoxComponent = ({ metadata }) => {
  // const extra = JSON.parse(token.extra);

  /** @todo: concat with the URL object */
  const image = base_ipfs + metadata.media;
  const rarity = metadata.extra.box_kind;

  const onClick = () => {
    console.log("clicked claim button");

    return props.onClaim(metadata.token_id);
  };

  return (
    <>
      <BoxTitle>{rarity} box</BoxTitle>
      <BoxRewardWrapper>
        <BoxRewardAmounts>
          <BoxRewardAmount>15 prizes</BoxRewardAmount>
          <BoxRewardAmount>1 prize</BoxRewardAmount>
          <BoxRewardAmount>200 prizes</BoxRewardAmount>
        </BoxRewardAmounts>
        <BoxRewardTitles>
          <BoxRewardTitle>Zomland NFT</BoxRewardTitle>
          <BoxRewardTitle>Nearnauts NFT</BoxRewardTitle>
          <BoxRewardTitle>100 Near token</BoxRewardTitle>
        </BoxRewardTitles>
      </BoxRewardWrapper>
      <BoxImage src={image} />
      <ClaimButton onClick={onClick}>Claim</ClaimButton>
    </>
  );
};

const OpenedBoxComponent = ({ metadata }) => {
  // const extra = JSON.parse(token.extra);

  /** @todo: concat with the URL object */
  const image = base_ipfs + metadata.media;
  const rarity = metadata.extra.box_kind;

  return (
    <>
      <BoxTitle>{rarity} box</BoxTitle>
      <OpenedBoxRewardTitle>50 near token</OpenedBoxRewardTitle>
      <BoxImage src={image} />
      <ClaimedButton>Claimed</ClaimedButton>
    </>
  );
};

const boxes = [
  {
    media: "bafkreibwmkcer2kp3kv67cydzhzzvzki7hdph5f4w7jeiep2r4s5dp7eb4",
    extra: {
      box_kind: "common",
    },
  },
  {
    media: "bafkreick7sjo4uzdy3sznvqjuafcds6f5p37apkggvvwkctptdy3qu2vbi",
    extra: {
      box_kind: "rare",
    },
  },
  {
    media: "bafkreigdv4mnfrndcob64wrwbqoqce257v7bvtxp2flnyqg2onukpssyoq",
    extra: {
      box_kind: "legendary",
    },
  },
];

return (
  <>
    <Wrapper>
      <WrapperContent>
        <Logo
          src="https://ipfs.near.social/ipfs/bafkreiht32vi4vui77rf7p42gchxmf5hjwjqbateehry4frovxhhrqpi7i"
          alt="Near Box Gray logo"
        />
        <PrimaryText>Congratulation!</PrimaryText>

        <SliderWrapper>
          <LeftArrow
            disabled={state.active === 0}
            onClick={previousActiveBox}
          />
          {boxes.map((box, index) => {
            const component =
              index === 1 ? (
                <OpenedBoxComponent key={index} metadata={box} />
              ) : (
                <ClosedBoxComponent key={index} metadata={box} />
              );

            return (
              <SingleBoxWrapper active={state.active === index}>
                {component}
              </SingleBoxWrapper>
            );
          })}
          <RightArrow
            disabled={state.active === boxes.length - 1}
            onClick={nextActiveBox}
          />
        </SliderWrapper>

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
  </>
);
