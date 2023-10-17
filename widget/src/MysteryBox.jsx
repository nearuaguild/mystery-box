const widget_owner_id = "denbite.testnet";

const account_id = context.accountId;
const contract_id = props.contract_id;

const createRewardsKey = (rarity) => `${rarity}_rewards`;

State.init({
  showVerificationScreen: false,
  boxes: {},
  [createRewardsKey("rare")]: [],
  [createRewardsKey("epic")]: [],
  [createRewardsKey("legendary")]: [],
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
  const data = Near.view(contract_id, "get_available_rewards", {
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
  const boxes = Near.view(contract_id, "get_account_boxes", {
    account_id: account_id,
  });

  if (boxes === undefined) throw `No boxes returned :(`;

  const entries = boxes.map((box) => [box.id, box]);

  const updated_boxes = Object.assign(
    {},
    state.boxes,
    Object.fromEntries(entries)
  );

  State.update({
    boxes: updated_boxes,
  });
};

try {
  fetchRewards(contract_id);
  fetchUserBoxes(contract_id, account_id);
} catch (err) {
  console.log("caught error on fetch rewards:", err);
  return (
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Screens.InvalidContract`}
    />
  );
}

const boxes = Object.values(state.boxes).map((box) => ({
  ...box,
  rewards: state[createRewardsKey(box.rarity)],
}));

console.log("boxes", boxes);

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

if (boxes.length === 0) {
  return (
    <Widget src={`${widget_owner_id}/widget/MysteryBox.Screens.NoBoxesFound`} />
  );
}

const redirectToVerification = () => {
  State.update({
    showVerificationScreen: true,
  });
};

const onClaim = (box_id) => {
  if (false) {
    return redirectToVerification();
  }

  
};

return (
  <Widget
    src={`${widget_owner_id}/widget/MysteryBox.Screens.Claim`}
    props={{
      onClaim,
      boxes,
    }}
  />
);
