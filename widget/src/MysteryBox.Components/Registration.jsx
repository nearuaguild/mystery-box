const widget_owner_id = "denbite.testnet";

const contract_id = props.contract_id;
const account_id = context.accountId;

// TODO: fetch if the account is already registered

const registry_account_id = Near.view(contract_id, "get_registry_iah_contract");
const issuer_account_id = Near.view(contract_id, "get_issuer_iah_contract");

console.log("registry_account_id", registry_account_id);
console.log("issuer_account_id", issuer_account_id);

const Wrapper = styled.div`
  margin: 25em;
`;

const Button = styled.button`
  background: white;

  font-size: 1.25em;
  padding: 0.25em 1em;
  border: 2px solid #f1c40f88;
  border-radius: 10px;
`;

const TextMessage = styled.p`
  font-size: 1.25em;
  color: #333;
`;

const isEligibleForRegistration = () => {
  const view = Near.view(registry_account_id, "sbt_tokens_by_owner", {
    account: account_id,
    issuer: issuer_account_id,
  });

  const sbtToken = view?.[0]?.[1]?.[0];

  return sbtToken !== undefined && sbtToken !== null;
};

const isEligible = true;
// const isEligible = isEligibleForRegistration();

console.log("eligible", isEligible);

if (isEligible === false) {
  return (
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Messages.VerificationRequired`}
    />
  );
}

const isAlreadyRegistered = Near.view(contract_id, "is_account_registered", {
  account_id: account_id,
});

const register = () => {
  Near.call(contract_id, "register");
};

if (isAlreadyRegistered === false) {
  return (
    <Wrapper>
      <Button onClick={register}>Register</Button>
    </Wrapper>
  );
} else if (isAlreadyRegistered === true) {
  return (
    <Wrapper>
      <TextMessage>You are already registered</TextMessage>
    </Wrapper>
  );
}
