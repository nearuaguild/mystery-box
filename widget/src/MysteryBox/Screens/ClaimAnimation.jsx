console.log("ClaimAnimation.props", props);

const font = fetch(
  "https://fonts.googleapis.com/css2?family=Lilita+One:wght@400&display=swap"
).body;

const font2 = fetch(
  "https://fonts.googleapis.com/css2?family=Kodchasan:wght@700&display=swap"
).body;

if (!font || !font2) {
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
  ${font2}
`;

const ContentWrapper = styled.div`
  ${(props) => {
    if (!props.landscape || !props.portrait)
      return `background-color: rgba(25, 25, 25, 0.25);`;

    return `
      @media (orientation: landscape) {
        background-image: url("${props.landscape}");
      }
    
      @media (orientation: portrait) {
        background-image: url("${props.portrait}");
      }
      `;
  }}

  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background-repeat: no-repeat;
  background-position: center;
  background-size: cover;
`;

const RewardBoundary = styled.div`
  width: 240px;
  height: 240px;
  border-radius: 30px;

  ${(props) => {
    if (props.rarity === "rare")
      return `
            border: 4px solid #4e70c9;
            box-shadow: 0 0 40px rgba(78, 112, 201, 0.4),
            inset 0 0 40px rgba(78, 112, 201, 0.4);
    `;

    if (props.rarity === "epic")
      return `
            border: 4px solid #8357AD;
            box-shadow: 0 0 40px rgba(131, 87, 173, 0.4),
            inset 0 0 40px rgba(131, 87, 173, 0.4);
    `;

    if (props.rarity === "legendary")
      return `
            border: 4px solid #D99B38;
            box-shadow: 0 0 40px rgba(217, 155, 56, 0.4),
            inset 0 0 40px rgba(217, 155, 56, 0.4);
    `;
  }}

  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;
`;

const NearRewardAmount = styled.div`
  padding: 0;
  margin: 0;

  color: #fff;
  text-align: center;
  font-family: Lilita One;
  font-size: 96px;
  line-height: 1;
  font-style: normal;
  font-weight: 400;
`;

const NonFungibleTokenRewardImage = styled.div`
  height: 88%;
  width: 88%;

  border-radius: 20px;

  ${(props) => {
    if (!props.src) return `background-color: rgba(255, 255, 255, 0.2);`;

    return `
    background-image: url(${props.src});
    background-size: cover;
    background-position: center;
    `;
  }}
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

const WhiteButton = styled.div`
  position: relative;
  cursor: pointer;

  text-align: none;
  text-decoration: none;

  @media (min-width: 512px) {
    font-size: 18px;
    width: 160px;
  }

  font-size: 16px;
  width: 120px;

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
  border: 3px solid rgb(255, 255, 255);
  box-shadow: 0 0 25px rgba(255, 255, 255, 0.3),
    inset 0 0 25px rgba(255, 255, 255, 0.3);
`;

const NearRewardIcon = (props) => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width={props.width}
      height={props.height}
      viewBox="0 0 39.434 10"
      fill="none"
    >
      <mask
        id="a"
        style={{ maskType: "luminance" }}
        maskUnits="userSpaceOnUse"
        x="0"
        y="0"
        width="209"
        height="53"
      >
        <path d="M39.434.048H0V9.97h39.434V.048Z" fill="#fff" />
      </mask>
      <g mask="url(#a)" fill="#fff">
        <path d="M16.34 1.583c-.728 0-1.257.171-1.707.567l-.794.685c-.066.052-.199.092-.292.014-.092-.079-.106-.185-.027-.291l.423-.633c.066-.092.014-.212-.106-.212h-1.018a.208.208 0 0 0-.212.211v6.168a.21.21 0 0 0 .212.212h1.058c.12 0 .212-.092.212-.211V4.614c0-1.595 1.336-1.845 1.839-1.845 1.072 0 1.454.764 1.454 1.345v3.981a.21.21 0 0 0 .212.212h1.058a.209.209 0 0 0 .212-.211V3.983c0-1.476-.966-2.399-2.526-2.399v-.002Zm6.839-.028c-2.051 0-3.36 1.252-3.36 2.952v.935c0 1.793 1.309 3.019 3.36 3.019 1.812 0 3.082-.935 3.214-2.201.013-.132-.079-.223-.212-.223H25.15a.199.199 0 0 0-.199.144c-.133.422-.753 1.054-1.773 1.054-1.019 0-1.972-.739-1.958-1.793l.013-1.174c.014-.883.939-1.489 1.945-1.489.913 0 1.798.514 1.891 1.357a.195.195 0 0 1-.157.205l-2.966.574a.269.269 0 0 0-.212.264v.014c0 .119.12.223.292.223h4.258a.212.212 0 0 0 .212-.212v-.83c0-1.568-1.362-2.821-3.32-2.821l.003.002Zm7.38.001c-1.653 0-3.082.962-3.082 2.228 0 .106.093.185.212.185h1.072c.106 0 .185-.079.199-.185.106-.579.807-1.002 1.561-1.002.9 0 1.508.554 1.508 1.502v1.147c0 1.174-.873 1.766-1.958 1.766-.846 0-1.336-.315-1.336-.831 0-.448.238-.831 1.217-1.054l1.415-.383c.145-.04.199-.158.172-.291-.014-.106-.132-.158-.238-.158h-1.468c-1.244 0-2.499.791-2.499 1.951v.185c0 1.185 1.124 1.805 2.408 1.805.821 0 1.522-.316 1.958-.685l.649-.554c.106-.092.212-.092.305 0 .079.079.052.198-.014.291l-.396.619c-.066.092-.013.212.106.212h.952c.12 0 .212-.092.212-.211V4.101c0-1.529-1.097-2.543-2.949-2.543h-.007Zm8.663.157h-1.481c-.516 0-1.018.316-1.375.62l-.581.5c-.066.052-.185.092-.265.027-.093-.065-.133-.198-.052-.304l.423-.633c.066-.092.014-.212-.106-.212h-.992a.208.208 0 0 0-.212.211v6.169a.21.21 0 0 0 .212.212h1.085c.12 0 .212-.092.212-.212V4.928c0-1.357.556-1.964 1.759-1.964h1.375c.12 0 .212-.092.212-.212v-.831a.21.21 0 0 0-.212-.212l-.002.003ZM8.898.048a1.066 1.066 0 0 0-.905.504L5.912 3.63a.219.219 0 0 0 .062.306.219.219 0 0 0 .266-.017l2.049-1.771a.085.085 0 0 1 .118.007.085.085 0 0 1 .022.055v5.543a.082.082 0 0 1-.147.052L2.088.422a1.068 1.068 0 0 0-.811-.374h-.215A1.06 1.06 0 0 0 0 1.105v7.808a1.06 1.06 0 0 0 1.062 1.058 1.066 1.066 0 0 0 .905-.504l2.081-3.079a.22.22 0 0 0-.06-.306.218.218 0 0 0-.266.017L1.673 7.87a.085.085 0 0 1-.118-.007.085.085 0 0 1-.022-.055V2.261a.083.083 0 0 1 .147-.052l6.193 7.389a1.065 1.065 0 0 0 .811.374H8.9c.586 0 1.062-.472 1.062-1.056v-7.81A1.06 1.06 0 0 0 8.9.048Z" />
      </g>
    </svg>
  );
};

const onClick = () => {
  console.log("ClaimAnimation", "Clicked button to go back");

  return props.onBack();
};

const LoadingReward = () => {
  return (
    <img src="https://ipfs.near.social/ipfs/bafkreideycaiegqdtadmktj2ljarutjwqnkoaun6vjjetne2qnoe7jstbq" />
  );
};

const NearReward = ({ amount }) => {
  const amountInNear = Big(Big(amount).div(1e24).toFixed(2))
    .toNumber()
    .toString();

  return (
    <RewardBoundary rarity={props.rarity}>
      <NearRewardAmount>{amountInNear}</NearRewardAmount>
      <NearRewardIcon width="80%" />
    </RewardBoundary>
  );
};

const getMediaUrlForToken = (contract_id, token_id) => {
  const metadata = Near.view(contract_id, "nft_metadata");
  const token = Near.view(contract_id, "nft_token", { token_id });

  if (!metadata || !token) return null;

  if (!metadata.base_uri || !token.metadata?.media) return undefined;

  const url = new URL(metadata.base_uri);
  url.pathname = token.metadata.media;

  return url.toString();
};

const NonFungibleTokenReward = ({ contract_id, token_id }) => {
  const url = getMediaUrlForToken(contract_id, token_id);

  return (
    <RewardBoundary rarity={props.rarity}>
      <NonFungibleTokenRewardImage src={url} />
    </RewardBoundary>
  );
};

const NothingReward = () => {
  return (
    <>
      <PrimaryText>
        Fortune has departed this time, but do not dismay!
      </PrimaryText>
      <PrimaryText>Join us next time and prepare for cosmic luck!</PrimaryText>
    </>
  );
};

const ContentWrapperComponent = ({ rarity, children }) => {
  const backgrounds = {
    rare: {
      portrait:
        "https://ipfs.near.social/ipfs/bafkreigdlbicksoqijiekfytogvz4cmykpbx2fdmpz5ghiqki2ou4dig2u",
      landscape:
        "https://ipfs.near.social/ipfs/bafkreibyfjqfivipdlavmjet2jdfvywkto2vi7ooi2eli6e4u3iprbwtoi",
    },
    epic: {
      portrait:
        "https://ipfs.near.social/ipfs/bafkreie32u3ab2ml5sqyu6oipo63h4mnbkytplqgcigqy4o5zxvznw4lwi",
      landscape:
        "https://ipfs.near.social/ipfs/bafkreihbthqvggathcry43suj6capp3es6bvhao4e7r73jmqipxvhedske",
    },
    legendary: {
      portrait:
        "https://ipfs.near.social/ipfs/bafkreidyj6fpphpuzve22lkyukqxicqr7sph5hjac4efakyzlzacmbfgbe",
      landscape:
        "https://ipfs.near.social/ipfs/bafkreicxafpy4qeqqh6ww6qkblvy3wc35jcddznjovtk2wbgmv6qamkyna",
    },
  };

  const background = backgrounds[rarity];

  return (
    <ContentWrapper
      portrait={background?.portrait}
      landscape={background?.landscape}
    >
      {children}
    </ContentWrapper>
  );
};

return (
  <Wrapper>
    <ContentWrapperComponent rarity={props.rarity}>
      <div
        style={{
          flexBasis: "55%",
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        {!props.reward && <LoadingReward />}
        {props.reward?.kind === "near" && (
          <NearReward amount={props.reward.amount} />
        )}
        {props.reward?.kind === "non_fungible_token" && (
          <NonFungibleTokenReward
            contract_id={props.reward.contract_id}
            token_id={props.reward.token_id}
          />
        )}
        {props.reward?.kind === "nothing" && <NothingReward />}
      </div>
      <WhiteButton onClick={onClick}>Back</WhiteButton>
    </ContentWrapperComponent>
  </Wrapper>
);
