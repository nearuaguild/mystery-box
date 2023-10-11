const widget_owner_id = "denbite.testnet";

let account_id = context.accountId;
let contract_id = props.contract_id;

console.log("account_id", account_id);
console.log("contract_id", contract_id);

State.init({
  verificationScreenShown: false,
});

// TODO: view spec to make sure it's appropriate contract
if (!contract_id) {
  return (
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Screens.InvalidContract`}
    />
  );
}

/** @todo if failed to retrieve rewards list (NoContractMethod) */
// if (!contract_id) {
//   return (
//     <Widget
//       src={`${widget_owner_id}/widget/MysteryBox.Screens.InvalidContract`}
//     />
//   );
// }

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

/** @todo if no rewards have been added to the contract */
// if (!registration_start_time) {
//   return (
//     <Widget
//       src={`${widget_owner_id}/widget/MysteryBox.Messages.ContestWasNotSetUp`}
//     />
//   );
// }

const boxes = [];

if (state.verificationScreenShown === true) {
  return (
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Screens.VerificationRequired`}
      props={{
        url: "https://i-am-human.app?community=nearukraine&vertical=regionalcommunities",
      }}
    />
  );
}

// if (boxes.length === 0) {
//   return (
//     <Widget
//       src={`${widget_owner_id}/widget/MysteryBox.Screens.NoBoxesFound`}
//       props={{
//         onClaim,
//         boxes,
//       }}
//     />
//   );
// }

const showVerificationScreen = () => {
  State.update({
    verificationScreenShown: true,
  });
};

const onClaim = (box_id) => {
  return showVerificationScreen();
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
