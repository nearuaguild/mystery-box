const widget_owner_id = "evasive-dime.testnet";

const { logInfo, logError } = VM.require(`${widget_owner_id}/widget/Utils.Logger`);

logInfo('props', props);

State.init({
  title: '',
});

const WrapperMenu = styled.div`
  background: rgba(24, 36, 50, 1);
  border: 0;
  border-radius: 30px;

  display: flex;
  flex-direction: column;
  justify-content: space-around;
  align-items: center;

  flex-basis: 50%;

  width: 48%;

  @media (max-width: 1024px) {
    width: 54%;
  }

  @media (max-width: 786px) {
    width: 70%;
  }

  @media (max-width: 512px) {
    width: 90%;
  }
`;

const MenuContent = styled.div`
  display: flex;
  flex-direction: column;

  justify-content: space-evenly;
  align-items: center;

  flex: 1;

  width: 100%;
`;

const FieldRow = styled.div`
  width: 80%;
  height: 72px;

  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;

  background: #27384c;

  border: 0;
  border-radius: 10px;
`;

const PrimaryText = styled.p`
  font-family: 'Kodchasan', sans-serif;
  font-size: 14px;
  font-weight: 500;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;

  padding: 0;
  margin: 0;
`;

const SecondaryText = styled.p`
  font-family: 'Kodchasan', sans-serif;
  font-size: 11px;
  font-weight: 400;
  letter-spacing: 0em;
  text-align: center;
  color: #2bccc2;

  padding: 0;
  margin: 0;
`;

const TextInput = styled.input`
  font-family: 'Kodchasan', sans-serif;
  font-size: 12px;
  font-weight: 400;
  letter-spacing: 0em;
  text-align: center;
  color: #2bccc2;

  background: #18243280;
  border: 0;
  border-radius: 50px;

  height: 72%;
  width: 90%;
`;

const WrapperHeader = styled.div`
  flex: 1;

  width: 90%;

  display: flex;
  justify-content: center;
  align-items: center;
`;

const WrapperContent = styled.div`
  flex: 1;

  width: 90%;

  display: flex;
  justify-content: center;
  align-items: center;
`;

function convertToSlug(text) {
  return text
    .toLowerCase()
    .replace(/[^\w ]+/g, '')
    .replace(/ +/g, '-');
}

const alias = state.title && convertToSlug(state.title).substring(0, 24);

const address = alias && alias + '.' + props.top_contract_id;

const shouldSubmitButtonBeDisabled = !state.title;

logInfo('shouldSubmitButtonBeDisabled', shouldSubmitButtonBeDisabled);

const submitTransactionToDeployContract = () => {
  logInfo('submitTransactionToDeployContract', state.title, address);

  const baseDeposit = Big(10).pow(24).mul(0.5); // 0.5 NEAR
  const argsDeposit = Big(240).mul(state.title.length).mul(Big(10).pow(18));

  Near.call(
    props.top_contract_id,
    'create_quest',
    {
      title: state.title,
    },
    Big(10).pow(12).mul(300), // 300 TGas
    baseDeposit.plus(argsDeposit)
  );
};

return (
  <>
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.Title`}
      props={{
        text: 'Create a Giveaway',
      }}
    />
    <WrapperMenu>
      <MenuContent>
        <FieldRow>
          <WrapperHeader>
            <PrimaryText>Giveaway Title</PrimaryText>
          </WrapperHeader>
          <WrapperContent>
            <TextInput
              value={state.title}
              placeholder="Tell us what this is gonna be about?"
              onChange={(e) => {
                State.update({
                  title: e.target.value,
                });
              }}
            />
          </WrapperContent>
        </FieldRow>
      </MenuContent>
    </WrapperMenu>
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.SubmitButton`}
      props={{
        text: 'Create',
        disabled: shouldSubmitButtonBeDisabled,
        onClick: submitTransactionToDeployContract,
      }}
    />
  </>
);
