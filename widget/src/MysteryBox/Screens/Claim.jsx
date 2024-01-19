console.log('Claim.props', props);

const base_ipfs = 'https://ipfs.near.social/ipfs/';

State.init({
  active: 0,
});

const font = fetch(
  'https://fonts.googleapis.com/css2?family=Kodchasan:wght@700&display=swap'
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

  flex-basis: 60%;
  width: 320px;
`;

const SingleBoxWrapper = styled.div`
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
  height: 100%;
  flex-basis: 80%;

  display: ${(props) => (props.active ? 'flex' : 'none')};
`;

const levitation = styled.keyframes`
  from {
    transform: translateY(0px) rotate(0deg);
  }
  to {
    transform: translateY(24px) rotate(6deg);
  }
`;

const BoxImageWrapper = styled.div`
  animation-duration: 3s;
  animation-iteration-count: ${(props) => (props.animate ? 'infinite' : '0')};
  animation-name: ${levitation};
  animation-timing-function: ease;
  animation-direction: alternate;
  text-align: center;

  flex-basis: 50%;

  svg {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 42px;
  }
`;

const BoxImage = styled.img`
  width: 75%;
  object-fit: cover;
`;

const BoxTitleWrapper = styled.div`
  flex-basis: 10%;

  display: flex;
  justify-content: center;
  align-items: center;
`;

const BoxTitle = styled.div`
  background: rgba(14, 18, 30, 0.6);
  border-radius: 50px;
  padding: 0.25em 3em;

  font-size: 14px;
  line-height: 20px;
  font-family: 'Kodchasan', sans-serif;
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
  justify-content: space-evenly;

  width: 100%;
  flex-basis: 18%;
`;

const BoxRewardAmounts = styled.div`
  display: flex;
  flex-direction: column;
  align-items: end;
  justify-content: space-between;
  margin: 0;
  padding: 0;
  flex-basis: 35%;
`;

const BoxRewardAmount = styled.p`
  overflow: hidden;
  white-space: nowrap; /* Don't forget this one */
  text-overflow: ellipsis;
  color: rgba(161, 224, 234, 1);
  font-family: 'Kodchasan', sans-serif;
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

  flex-basis: 50%;
`;

const BoxRewardTitle = styled.p`
  overflow: hidden;
  white-space: nowrap; /* Don't forget this one */
  text-overflow: ellipsis;
  color: rgba(255, 255, 255, 1);
  font-family: 'Kodchasan', sans-serif;
  font-size: 12px;
  font-weight: 700;
  line-height: 18px;
  letter-spacing: 0em;
  text-align: center;
  text-transform: uppercase;
  margin: 0;
  padding: 0;
`;

const BoxLockedTitle = styled.p`
  color: #a1e0ea;
  font-family: 'Kodchasan', sans-serif;
  font-size: 18px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  text-transform: uppercase;
  margin: 0;
  padding: 0;
`;

const BoxBottomWrapper = styled.div`
  flex-basis: 12%;

  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
`;

const ClaimButton = styled.div`
  position: relative;

  cursor: pointer;
  text-align: none;
  text-decoration: none;

  @media (min-width: 512px) {
    font-size: 20px;
    padding: 0.65em 2.5em;
  }

  font-size: 16px;
  line-height: 1;
  padding: 0.6875em 2.5em;

  font-family: 'Kodchasan', sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;

  background: none;

  &:after {
    content: '';
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
  font-family: 'Kodchasan', sans-serif;
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

  font-family: 'Kodchasan', sans-serif;
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

const Logo = styled.img`
  height: 55px;
`;

const BottomText = styled.p`
  margin: 0;

  color: white;

  font-size: 14px;
  font-family: 'Kodchasan', sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  text-transform: uppercase;
`;

const WrapperSocial = styled.div`
  display: flex;
  width: 180px;
  align-items: center;
  justify-content: space-around;
`;

const SocialText = styled.p`
  font-family: 'Kodchasan', sans-serif;
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
  if (state.active === props.boxes.length - 1) return;

  State.update({ active: state.active + 1 });
};

const NonClaimedBoxComponent = ({ box }) => {
  /** @todo: concat with the URL object */
  const image = base_ipfs + box.ipfs;
  const rarity = box.rarity;

  const onClick = () => {
    console.log('clicked claim button', box.id);

    return props.onClaim(box.id);
  };

  const amounts = (box.rewards || []).map((reward) => {
    const count = reward.available || reward.token_ids?.length;

    const isPlural = count > 1;

    const title = isPlural ? 'prizes' : 'prize';

    return `${count} ${title}`;
  });

  const titles = (box.rewards || []).map((reward) => {
    if (reward.kind === 'near') {
      const amountInNear = Big(
        Big(reward.amount).div(1e24).toFixed(2)
      ).toNumber();

      return `${amountInNear} near token`;
    } else if (reward.kind === 'non_fungible_token') {
      const { name } = Near.view(reward.contract_id, 'nft_metadata');

      const maximumNameLength = 12;
      const shortName =
        name.length > maximumNameLength
          ? name.substring(0, maximumNameLength) + '...'
          : name;

      return `${shortName} nft`;
    }
  });

  return (
    <>
      <BoxTitleWrapper>
        <BoxTitle>{rarity} box</BoxTitle>
      </BoxTitleWrapper>
      <BoxRewardWrapper>
        <BoxRewardAmounts>
          {amounts.map((text) => (
            <BoxRewardAmount>{text}</BoxRewardAmount>
          ))}
        </BoxRewardAmounts>
        <BoxRewardTitles>
          {titles.map((text) => (
            <BoxRewardTitle>{text}</BoxRewardTitle>
          ))}
        </BoxRewardTitles>
      </BoxRewardWrapper>
      <BoxImageWrapper animate>
        <BoxImage src={image} />
      </BoxImageWrapper>
      <BoxBottomWrapper>
        <ClaimButton onClick={onClick}>Claim</ClaimButton>
      </BoxBottomWrapper>
    </>
  );
};

const LockIcon = ({}) => {
  return (
    <svg
      width="69"
      height="88"
      viewBox="0 0 69 88"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M29.644 87.956c-7.714-.479-13.857-1.268-20.106-2.582-2.028-.426-7.145-1.714-8.713-2.192l-.774-.237-.044-19.414C-.017 52.854.022 44.06.093 43.99c.259-.255 6.755-1.897 10.453-2.642 16.104-3.247 30.935-3.295 46.972-.15 2.984.585 8.008 1.791 10.151 2.438l1.032.311v39.035l-2.666.734c-6.868 1.89-14.393 3.227-22.024 3.91-2.848.256-12.134.469-14.367.33ZM36.96 70.38v-4.258l.98-.424c1.404-.61 3.156-2.315 3.897-3.798 1.653-3.304 1.04-7.006-1.599-9.644-3.364-3.365-8.366-3.365-11.73 0-2.638 2.638-3.252 6.34-1.6 9.644.742 1.483 2.494 3.189 3.899 3.798l.98.424v8.517h5.161V70.38h.012Zm-3.703-9.246c-2.143-.753-2.755-3.612-1.122-5.245 1.764-1.764 4.746-.925 5.328 1.5.282 1.18.02 2.1-.848 2.967-.979.978-2.067 1.23-3.358.777ZM7.06 30.4c.063-5.807.107-6.613.452-8.285C9.702 11.485 17.604 3.276 28.096.728c6.046-1.468 12.782-.692 18.41 2.12 7.626 3.812 12.943 10.768 14.743 19.287.307 1.45.361 2.486.435 8.181.076 5.955.06 6.511-.195 6.414-.512-.197-6.352-1.288-9.004-1.682l-2.624-.39v-4.603c0-4.817-.107-5.818-.853-7.968-1.304-3.756-4.482-7.208-8.18-8.886-2.265-1.026-3.528-1.276-6.452-1.276-2.256 0-2.873.06-4.007.388-5.854 1.695-9.978 6.09-11.229 11.967-.183.86-.25 2.412-.25 5.807 0 3.448-.051 4.634-.202 4.634-.862 0-10.701 1.73-11.43 2.01-.24.093-.261-.56-.199-6.33Z"
        fill="#fff"
        fill-opacity=".3"
      />
    </svg>
  );
};

const LockedBoxComponent = ({ box }) => {
  /** @todo: concat with the URL object */
  const image = base_ipfs + box.ipfs;
  const rarity = box.rarity;

  return (
    <>
      <BoxTitleWrapper>
        <BoxTitle>{rarity} box</BoxTitle>
      </BoxTitleWrapper>
      <BoxRewardWrapper />
      <BoxImageWrapper>
        <BoxImage src={image} />
        <LockIcon />
      </BoxImageWrapper>
      <BoxBottomWrapper>
        <BoxLockedTitle>Rewards out of stock</BoxLockedTitle>
      </BoxBottomWrapper>
    </>
  );
};

const OpenedBoxComponent = ({ box }) => {
  /** @todo: concat with the URL object */
  const image = base_ipfs + box.ipfs;
  const rarity = box.rarity;

  const reward = box.status.reward;

  let text;

  if (reward.kind === 'near') {
    const amountInNear = Big(
      Big(reward.amount).div(1e24).toFixed(2)
    ).toNumber();

    text = `${amountInNear} near token`;
  } else if (reward.kind === 'non_fungible_token') {
    const { name } = Near.view(reward.contract_id, 'nft_metadata');

    text = `${name} nft`;
  } else if (reward.kind === 'nothing') {
    text = 'Better luck next time';
  }

  return (
    <>
      <BoxTitleWrapper>
        <BoxTitle>{rarity} box</BoxTitle>
      </BoxTitleWrapper>
      <BoxRewardWrapper>
        <OpenedBoxRewardTitle>{text}</OpenedBoxRewardTitle>
      </BoxRewardWrapper>
      <BoxImageWrapper>
        <BoxImage src={image} />
      </BoxImageWrapper>
      <BoxBottomWrapper>
        <ClaimedButton>Claimed</ClaimedButton>
      </BoxBottomWrapper>
    </>
  );
};

const BoxComponent = ({ box }) => {
  if (box.status.kind === 'claimed')
    return <OpenedBoxComponent key={box.id} box={box} />;

  if (box.status.kind === 'non_claimed' && box.rewards.length === 0)
    return <LockedBoxComponent key={box.id} box={box} />;

  if (box.status.kind === 'non_claimed')
    return <NonClaimedBoxComponent key={box.id} box={box} />;

  return <></>;
};

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

const available = (props.boxes || []).filter(
  (box) => box.status.kind === 'non_claimed' && box.rewards.length > 0
).length;

return (
  <>
    <Wrapper>
      <WrapperContent>
        <Logo
          src="https://ipfs.near.social/ipfs/bafkreiht32vi4vui77rf7p42gchxmf5hjwjqbateehry4frovxhhrqpi7i"
          alt="Near Box Gray logo"
        />
        <SliderWrapper>
          <LeftArrow
            disabled={state.active === 0}
            onClick={previousActiveBox}
          />
          {props.boxes.map((box, index) => {
            return (
              <SingleBoxWrapper active={state.active === index}>
                <BoxComponent box={box} />
              </SingleBoxWrapper>
            );
          })}
          <RightArrow
            disabled={state.active === props.boxes.length - 1}
            onClick={nextActiveBox}
          />
        </SliderWrapper>

        <BottomText>
          <span style={{ color: '#2bccc2' }}>{props.totalSupply}</span>
          minted boxes
          <br />
          <span style={{ color: available > 0 ? '#FFD951' : '#2bccc2' }}>
            {available}
          </span>
          box{available === 1 ? '' : 'es'} available
        </BottomText>

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
