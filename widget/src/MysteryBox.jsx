const widget_owner_id = "denbite.testnet";
const rpc_endpoint = "https://rpc.testnet.near.org";

const account_id = context.accountId;
const contract_id = props.contract_id;

const createRewardsKey = (rarity) => `${rarity}_rewards`;

State.init({
  showVerificationScreen: false,
  showClaimAnimationScreen: false,
  boxes: {},
  [createRewardsKey("rare")]: [],
  [createRewardsKey("epic")]: [],
  [createRewardsKey("legendary")]: [],
  totalSupply: "0",
  lastClaimedBoxReward: null,
  lastClaimedBoxRarity: null,
});

// TODO: view spec to make sure it's appropriate contract
if (!contract_id) {
  return (
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Screens.InvalidContract`}
    />
  );
}

// TODO: remove styled containers for components to make them importable (only pages may contain containers)
if (!account_id) {
  return (
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Screens.AuthenticationRequired`}
      props={{
        url: "https://near.org/signin",
      }}
    />
  );
}

const fetchRewards = (contract_id) => {
  fetchAndUpdateRewardsByRarity(contract_id, "rare");
  fetchAndUpdateRewardsByRarity(contract_id, "epic");
  fetchAndUpdateRewardsByRarity(contract_id, "legendary");
};

const fetchAvailableRewardsByRarity = (contract_id, rarity) => {
  const data = Near.view(contract_id, "available_rewards", {
    rarity: rarity,
    pagination: {
      page: 1,
      size: 3,
    },
  });

  if (data === undefined) throw `No ${rarity} rewards returned :(`;

  return data || [];
};

const fetchAndUpdateRewardsByRarity = (contract_id, rarity) => {
  const key = createRewardsKey(rarity);

  if (state[key].length !== 0) return;

  const rewards = fetchAvailableRewardsByRarity(contract_id, rarity);

  if (rewards.length === 0) return;

  const updated_rewards = [...state[key], ...rewards];

  State.update({
    [key]: updated_rewards,
  });
};

const fetchUserBoxes = (contract_id, account_id) => {
  const boxes = Near.view(contract_id, "boxes_for_owner", {
    account_id: account_id,
    pagination: {
      page: 1,
      size: 20,
    },
  });

  if (boxes === undefined) throw `No boxes returned :(`;

  const entries = (boxes || []).map((box) => [box.id, box]);

  const updated_boxes = Object.assign(
    {},
    state.boxes,
    Object.fromEntries(entries)
  );

  State.update({
    boxes: updated_boxes,
  });
};

const fetchTotalSupply = (contract_id) => {
  const totalSupply = Near.view(contract_id, "total_supply", {});

  if (totalSupply === undefined) throw `No supply returned :(`;

  State.update({
    totalSupply,
  });
};

const fetchTransactionByHash = (hash, sender_id) => {
  return fetch(rpc_endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      jsonrpc: "2.0",
      id: "dontcare",
      method: "tx",
      params: [hash, sender_id],
    }),
  });
};

const parseResultFromClaimTransactionResponse = (response) => {
  if (!response?.body) throw `Response is missing body`;

  if (response.body.error) throw response.body.error.data || "Unknown error";

  const result = response.body.result;

  if (!result) throw `Body is missing result field`;

  const responseValue = result?.status?.SuccessValue;

  if (!responseValue) return null;

  console.log("result", result);

  return JSON.parse(Buffer.from(responseValue, "base64").toString());
};

const fetchClaimTransactionResult = (hash, account_id) => {
  try {
    const response = fetchTransactionByHash(hash, account_id);

    const result = parseResultFromClaimTransactionResponse(response);

    console.log("claim result", result);

    if (!result) return;

    Storage.set(hash, 1);

    State.update({
      showClaimAnimationScreen: true,
      lastClaimedBoxRarity: result[1],
    });

    setTimeout(() => {
      State.update({
        lastClaimedBoxReward: result[2],
      });
    }, 2_000);
  } catch (error) {
    console.warn(`Caught error during fetch claim tx result`, error);

    State.update({
      showClaimAnimationScreen: false,
    });
  }
};

try {
  fetchRewards(contract_id);
  fetchUserBoxes(contract_id, account_id);
  fetchTotalSupply(contract_id);
} catch (err) {
  console.log("caught error on fetch rewards:", err);
  return (
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Screens.InvalidContract`}
    />
  );
}

if (state.showVerificationScreen === true) {
  return (
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Screens.VerificationRequired`}
      props={{
        url: "https://i-am-human.app?community=nearukraine&vertical=regionalcommunities",
      }}
    />
  );
}

const value = Storage.get(props.transactionHashes);
const hashExistInStorage = value !== null && value !== undefined;
console.log("hashExistInStorage", hashExistInStorage);

try {
  if (props.transactionHashes && !hashExistInStorage) {
    fetchClaimTransactionResult(props.transactionHashes, account_id);
  }
} catch (err) {
  console.warn("caught error on fetch claim transaction result:", err);
}

if (state.showClaimAnimationScreen === true) {
  const onBack = () => {
    State.update({
      showClaimAnimationScreen: false,
    });
  };

  return (
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Screens.ClaimAnimation`}
      props={{
        reward: state.lastClaimedBoxReward,
        rarity: state.lastClaimedBoxRarity,
        onBack,
      }}
    />
  );
}

const boxes = Object.values(state.boxes).map((box) => ({
  ...box,
  rewards: state[createRewardsKey(box.rarity)],
}));

if (boxes.length === 0) {
  return (
    <Widget src={`${widget_owner_id}/widget/MysteryBox.Screens.NoBoxesFound`} />
  );
}

const getRegistryIAHContract = (currect_contract_id) => {
  if (currect_contract_id.endsWith(".near")) return "registry.i-am-human.near";

  return `registry-v2.i-am-human.testnet`;
};

const getIssuerIAHContract = (currect_contract_id) => {
  if (currect_contract_id.endsWith(".near")) return "fractal.i-am-human.near";

  return `fractal-v2.i-am-human.testnet`;
};

const checkVerification = (account_id) => {
  const view = Near.view(
    getRegistryIAHContract(widget_owner_id),
    "sbt_tokens_by_owner",
    {
      account: account_id,
      issuer: getIssuerIAHContract(widget_owner_id),
    }
  );

  const sbtToken = view?.[0]?.[1]?.[0];

  return sbtToken !== undefined && sbtToken !== null;
};

const isVerified = checkVerification(account_id);

const redirectToVerification = () => {
  State.update({
    showVerificationScreen: true,
  });
};

const onClaim = (box_id) => {
  // if (!isVerified) {
  //   return redirectToVerification();
  // }

  const gas = Big(100e12).toString(); // 100 TGas

  return Near.call(
    contract_id,
    "claim",
    {
      box_id,
    },
    gas,
    1 // exactly 1 yocto
  );
};

return (
  <Widget
    src={`${widget_owner_id}/widget/MysteryBox.Screens.Claim`}
    props={{
      onClaim,
      boxes,
      totalSupply: state.totalSupply,
    }}
  />
);
