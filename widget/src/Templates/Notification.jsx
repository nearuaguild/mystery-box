const widget_owner_id = "evasive-dime.testnet";

const { logInfo, logError } = VM.require(`${widget_owner_id}/widget/Utils.Logger`);

const rpc_endpoint = 'https://rpc.testnet.near.org';

State.init({
  title: null,
  subtitle: null,
  variant: null,
});

const NotificationWrapper = styled.div`
  position: absolute;
  top: 10px;
  right: 10px;

  height: 54px;
  width: 360px;
  max-width: 90%;

  z-index: 999;
`;

const getTitleFromMethod = (method) => {
  switch (method) {
    case 'mint':
    case 'mint_many':
      return 'Minting was successful';
    case 'add_near_reward':
      return 'Adding NEAR reward was successful';
    case 'nft_transfer_call':
      return 'Adding NFT reward was successful';
    case 'create_quest':
      return 'Giveaway has been created';
    default:
      return '';
  }
};

const fetchTransactionByHash = (hash, sender_id) => {
  return fetch(rpc_endpoint, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      jsonrpc: '2.0',
      id: 'dontcare',
      method: 'tx',
      params: [hash, sender_id],
    }),
  });
};

const parseResultFromTransactionResponse = (response) => {
  if (!response?.body) throw `Response is missing body`;

  if (response.body.error) throw response.body.error.data || 'Unknown error';

  const result = response.body.result;

  if (!result) throw `Body is missing result field`;

  return result;
};

const fetchTransactionResult = (hash, account_id) => {
  try {
    const response = fetchTransactionByHash(hash, account_id);

    const result = parseResultFromTransactionResponse(response);

    logInfo('tx result', result);

    if (!result) return;

    const method = (result.transaction?.actions || [])?.[0]?.['FunctionCall']
      .method_name;

    const value = typeof result?.status?.SuccessValue === 'string';

    Storage.set(hash, 1);

    if (!method || !value) return;

    const title = getTitleFromMethod(method);
    const variant = 'success';

    State.update({
      title,
      variant,
    });

    setTimeout(() => {
      State.update({
        title: null,
        subtitle: null,
        variant: null,
      });
    }, 6_000);
  } catch (error) {
    console.warn(`Caught error during fetch tx result`, error);

    State.update({
      title: null,
      subtitle: null,
      variant: null,
    });
  }
};

const value = Storage.get(props.tx_hash);

logInfo('value', value);
const hashExistInStorage = value !== null && value !== undefined;

logInfo('hashExistInStorage', hashExistInStorage);

try {
  if (props.tx_hash && !hashExistInStorage) {
    fetchTransactionResult(props.tx_hash, context.accountId);
  }
} catch (err) {
  console.warn('caught error on fetch claim transaction result:', err);
}

const showNotification = !!state.title && !!state.variant;

if (!showNotification) return <></>;

return (
  <NotificationWrapper>
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.Notification`}
      props={{
        title: state.title,
        subtitle: state.subtitle,
        variant: state.variant,
        onClose: () => {
          State.update({
            title: null,
            subtitle: null,
            variant: null,
          });
        },
      }}
    />
  </NotificationWrapper>
);
