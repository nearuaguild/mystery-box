console.log('props', props);

const isValidAccount = (account) => {
  if (account.amount === undefined || account.amount === null) return false;

  const amountNum = Number(account.amount);

  if (!Number.isInteger(amountNum)) return false;

  if (amountNum < 1) return false;

  if (!account.address) return false;

  /** @todo verify address */

  return true;
};

const createAccount = () => ({
  id: Date.now(),
  rarity: 'rare',
  address: '',
  amount: '1',
});

State.init({
  accounts: [createAccount()],
});

const WrapperMenu = styled.div`
  background: rgba(24, 36, 50, 1);
  border: 0;
  border-radius: 30px;

  display: flex;
  flex-direction: column;
  justify-content: space-around;
  align-items: center;

  flex-basis: 70%;

  width: 40%;

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

const MenuTitle = styled.p`
  font-family: 'Kodchasan', sans-serif;
  font-size: 28px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  margin: 0;
`;

const MenuSubtitle = styled.p`
  font-family: 'Kodchasan', sans-serif;
  font-size: 14px;
  font-weight: 400;
  letter-spacing: 0em;
  text-align: center;
  color: rgba(43, 204, 194, 1);
  margin: 0;
`;

const MenuHeader = styled.div``;
const MenuContent = styled.div`
  flex-basis: 230px;

  @media (max-width: 512px) {
    flex-basis: 320px;
  }

  width: 100%;

  overflow-x: hidden;
  overflow-y: scroll;
`;

const WrapperMenuButton = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
`;

const WrapperAddPoolButton = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
`;

const AddPoolButton = styled.button`
  border: 0;
  border-radius: 10px;

  width: 60%;
  height: 24px;

  font-family: 'Kodchasan', sans-serif;
  font-size: 14px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;

  background: #638caf33;
`;

const WrapperTable = styled.li`
  display: flex;
  flex-direction: row;
  justify-content: space-evenly;

  width: 100%;
  margin-bottom: 16px;
`;

const TableContainer = styled.div`
  display: flex;
  flex-direction: column;

  flex-basis: 80%;

  height: 70px;

  background: #27384c;

  border: 0;
  border-radius: 10px;
`;

const WrapperRarityButton = styled.div`
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: 0;

  ${(props) => (props.active ? 'border: 1px solid #8DBFEA;' : 'border: 0;')}

  background: #202f3f;

  display: flex;
  justify-content: center;
  align-items: center;

  &:hover {
    cursor: pointer;
  }
`;

const RarityButton = styled.div`
  width: 14px;
  height: 14px;
  border-radius: 50%;
  border: 0;

  ${(props) => {
    if (props.rarity === 'rare') return `background: #1EA3AF;`;
    if (props.rarity === 'epic') return `background: #B263C3;`;
    if (props.rarity === 'legendary') return `background: #FBC70F;`;
  }}
`;

const HeaderRow = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;

  background: #334a5f;
  height: 30px;

  border: 0;
  border-radius: 10px;
`;

const TableRow = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;

  flex-grow: 1;
`;

const TableCell = styled.div`
  flex: 1;
  text-align: center;

  font-family: 'Kodchasan', sans-serif;
  font-size: 11px;
  font-weight: 300;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;

  display: flex;
  justify-content: space-evenly;
  align-items: center;
`;

const TextCell = styled.input`
  font-family: 'Kodchasan', sans-serif;
  font-size: 11px;
  font-weight: 400;
  letter-spacing: 0em;
  text-align: center;
  color: #2bccc2;
  width: 110px;
  height: 21px;

  background: #18243280;
  border: 0;
  border-radius: 50px;
`;

const NumberCell = styled.input`
  font-family: 'Kodchasan', sans-serif;
  font-size: 14px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #2bccc2;
  width: 80px;
  height: 21px;

  background: #18243280;
  border: 0;
  border-radius: 50px;

  ::-webkit-outer-spin-button,
  ::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  -moz-appearance: textfield;
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

const deleteAccount = (id) => {
  console.log('deleting account', id);

  State.update((previousState) => {
    return {
      ...previousState,
      accounts: previousState.accounts.filter((account) => account.id !== id),
    };
  });
};

const addAccount = () => {
  console.log('add new account');

  State.update((previousState) => {
    return {
      ...previousState,
      accounts: [...previousState.accounts, createAccount()],
    };
  });
};

const updadeAccount = (id, field, value) => {
  console.log(`updating account.${field}`, id, value);

  State.update((previousState) => {
    const { accounts } = previousState;

    const account = accounts.find((account) => account.id === id);

    if (account === -1) return previousState;

    // unknown field
    if (account[field] === 'undefined') {
      console.error(
        `Can't update account property ${field} since it doesn't exist`
      );

      return previousState;
    }

    account[field] = value;

    return {
      ...previousState,
      accounts,
    };
  });
};

const accounts = state.accounts || [];

const everyAccountIsValid = accounts.every(isValidAccount);

const shouldSubmitButtonBeDisabled = !everyAccountIsValid;

console.log('shouldSubmitButtonBeDisabled', shouldSubmitButtonBeDisabled);

const submitTransactionToMintBoxes = () => {
  console.log('submitTransactionToMintBoxes', accounts);

  if (!everyAccountIsValid) return;

  const accountsGrouppedByRarity = accounts.reduce((prev, curr) => {
    const { rarity, ...other } = curr;

    const previousAccounts = prev[rarity] || [];

    return {
      ...prev,
      [rarity]: [...previousAccounts, other],
    };
  }, {});

  console.log('accountsGrouppedByRarity', accountsGrouppedByRarity);

  const txn = Object.entries(accountsGrouppedByRarity).map(
    ([rarity, partialAccounts]) => {
      const accounts = partialAccounts
        .map((partialAccount) =>
          Array(parseInt(partialAccount.amount)).fill(partialAccount.address)
        )
        .flat();

      const total = accounts.reduce((prev, curr) => {
        return prev + 2520 + 40 * curr.length;
      }, 0);

      console.log('total', total);

      const totalNum = Big(680).plus(total).mul(Big(10).pow(18));

      return {
        contractName: props.contract_id,
        methodName: 'mint_many',
        args: {
          rarity,
          accounts,
        },
        gas: Big(10).pow(14), // 100 TGas
        deposit: totalNum.toFixed(), // 0.00064N + accounts
      };
    }
  );

  Near.call(txn);
};

return (
  <>
    <Widget
      src="denbite.testnet/widget/MysteryBox.Manage.Components.Title"
      props={{
        text: 'Mint Box',
      }}
    />
    <WrapperMenu>
      <MenuHeader>
        <MenuTitle>Contract Name</MenuTitle>
        <MenuSubtitle>{props.contract_id}</MenuSubtitle>
      </MenuHeader>
      <MenuContent>
        {accounts.map((account) => (
          <WrapperTable key={`account_unique_key_${account.id}`}>
            <Widget
              src="denbite.testnet/widget/MysteryBox.Manage.Components.DeleteButton"
              props={{
                onClick: () => deleteAccount(account.id),
              }}
            />
            <TableContainer>
              <HeaderRow>
                <TableCell>Box Rarity</TableCell>
                <TableCell>Account</TableCell>
                <TableCell>Amount</TableCell>
              </HeaderRow>
              <TableRow>
                <TableCell>
                  <WrapperRarityButton
                    active={account.rarity === 'rare'}
                    disabled={account.rarity === 'rare'}
                    onClick={() => {
                      updadeAccount(account.id, 'rarity', 'rare');
                    }}
                  >
                    <RarityButton rarity="rare" />
                  </WrapperRarityButton>
                  <WrapperRarityButton
                    active={account.rarity === 'epic'}
                    disabled={account.rarity === 'epic'}
                    onClick={() => {
                      updadeAccount(account.id, 'rarity', 'epic');
                    }}
                  >
                    <RarityButton rarity="epic" />
                  </WrapperRarityButton>
                  <WrapperRarityButton
                    active={account.rarity === 'legendary'}
                    disabled={account.rarity === 'legendary'}
                    onClick={() => {
                      updadeAccount(account.id, 'rarity', 'legendary');
                    }}
                  >
                    <RarityButton rarity="legendary" />
                  </WrapperRarityButton>
                </TableCell>
                <TableCell>
                  <TextCell
                    value={account.address}
                    onChange={(e) => {
                      updadeAccount(account.id, 'address', e.target.value);
                    }}
                  />
                </TableCell>
                <TableCell>
                  <NumberCell
                    type="number"
                    min="1"
                    lang="en"
                    step="1"
                    value={account.amount}
                    onChange={(e) => {
                      updadeAccount(account.id, 'amount', e.target.value);
                    }}
                  />
                </TableCell>
              </TableRow>
            </TableContainer>
          </WrapperTable>
        ))}
        <WrapperAddPoolButton>
          <AddPoolButton onClick={addAccount}>Add account</AddPoolButton>
        </WrapperAddPoolButton>
      </MenuContent>
    </WrapperMenu>
    <Widget
      src={`denbite.testnet/widget/MysteryBox.Manage.Components.SubmitButton`}
      props={{
        text: 'Submit',
        disabled: shouldSubmitButtonBeDisabled,
        onClick: submitTransactionToMintBoxes,
      }}
    />
  </>
);
