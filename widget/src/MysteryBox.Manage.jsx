const widget_owner_id = 'evasive-dime.testnet';
const top_contract_id = 'boundless-berry.testnet';

const { logInfo } = VM.require(`${widget_owner_id}/widget/Utils.Logger`);
const rpc_endpoint = 'https://rpc.testnet.near.org';

console.log("MysteryBox.Manage props", props);

logInfo("ETREMELLY IMPORTANT INFORMATION");

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

const parseResultFromClaimTransactionResponse = (response) => {
  if (!response?.body) throw `Response is missing body`;

  if (response.body.error) throw response.body.error.data || 'Unknown error';

  const result = response.body.result;

  if (!result) throw `Body is missing result field`;

  const responseValue = result?.status?.SuccessValue;

  if (!responseValue) return null;

  console.log('result', result);

  return JSON.parse(Buffer.from(responseValue, 'base64').toString());
};

const account_id = context.accountId;

const KnownPages = [
  'Home',
  'AddNearReward',
  'AddNftReward',
  'MintBox',
  'ListRewards',
  'ListUserBoxes',
  'DeployContract',
];

const determinePageFromProps = () => {
  if (!account_id) return 'SignIn';

  if (!KnownPages.includes(props.page)) return 'Home';

  if (props.page === 'DeployContract' && props.transactionHashes) {
    const response = fetchTransactionByHash(
      props.transactionHashes,
      account_id
    );

    console.log('response', response);

    const result = parseResultFromClaimTransactionResponse(response);

    console.log('result', result);

    if (result) {
      return 'Home';
    }
  }

  return props.page;
};

const page = determinePageFromProps();

// Import our modules
const { Layout } = VM.require(`${widget_owner_id}/widget/Templates.Layout`);

if (!Layout) {
  return <p>Loading modules...</p>;
}

const { href: linkHref } = VM.require(`${widget_owner_id}/widget/core.lib.url`);

linkHref || (linkHref = () => {});

function Page({ page, account_id, quest_id }) {
  if (page === 'SignIn') {
    return (
      <Widget
        src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryText`}
        props={{
          text: 'Please sign in with your near wallet to proceed',
        }}
      />
    );
  }

  if (page === 'DeployContract') {
    return (
      <Widget
        src={`${widget_owner_id}/widget/MysteryBox.Manage.Screens.DeployContract`}
        props={{
          top_contract_id,
        }}
      />
    );
  }

  const quests =
    Near.view(top_contract_id, 'quests_per_owner', {
      account_id,
    }) || [];

  const currentQuest = !isNaN(quest_id) ? quests.find((quest) => quest.quest_id.toString() === quest_id) : null;

  console.log("MysteryBox.Manage quests", quests, quest_id, currentQuest);

  switch (page) {
    case 'Home': {
      if (quests.length === 0) {
        return (
          <>
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryText`}
              props={{
                text: `
                Ready for an adventure?
                Click below to create a new contract and join the Mystery Box community!
                `,
              }}
            />
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryLinkButton`}
              props={{
                text: 'Create new contract',
                href: linkHref({
                  widgetSrc: `${widget_owner_id}/widget/MysteryBox.Manage`,
                  params: {
                    quest_id,
                    page: 'DeployContract',
                  },
                }),
              }}
            />
          </>
        );
      }

      return (
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Screens.Home`}
          props={{
            quests: quests,
          }}
        />
      );
    }
    case 'AddNftReward': {
      const contracts = Near.view(quest_id, 'trusted_nft_contracts');

      console.log('contracts', contracts);

      const tokens = (contracts || [])
        .map((contract) => {
          const metadata = Near.view(contract, 'nft_metadata');

          const tokens = Near.view(contract, 'nft_tokens_for_owner', {
            account_id,
          });

          return (tokens || []).map((token) => ({
            contract,
            metadata,
            token,
          }));
        })
        .flat();

      if (tokens.length === 0)
        return (
          <Widget
            src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryText`}
            props={{
              text: `
              NFT rewards are supported only from trusted collections!
Please reach out to Near Ukraine Team in order to have your collection verified
`,
            }}
          />
        );

      return (
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Screens.AddNftReward`}
          props={{
            contract: currentQuest,
            tokens,
          }}
        />
      );
    }
    case 'ListRewards': {
      /** @todo fetch rarity from backend */

      const fetchRewards = (rarity) => {
        const rewards = Near.view(quest_id, 'rewards', {
          rarity,
        });

        return (rewards || []).map((reward) => ({
          ...reward,
          rarity,
        }));
      };

      const rewards = [
        fetchRewards('rare'),
        fetchRewards('epic'),
        fetchRewards('legendary'),
      ].flat();

      if (rewards.length === 0)
        return (
          <>
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryText`}
              props={{
                text: 'No rewards have been added so far',
              }}
            />
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryLinkButton`}
              props={{
                text: 'Add first NEAR reward',
                href: linkHref({
                  widgetSrc: `${widget_owner_id}/widget/MysteryBox.Manage`,
                  params: {
                    quest_id,
                    page: 'AddNearReward',
                  },
                }),
              }}
            />
          </>
        );

      return (
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Screens.ListRewards`}
          props={{
            contract: currentQuest,
            rewards,
          }}
        />
      );
    }
    case 'ListUserBoxes': {
      /** @todo fetch addresses from backend */
      const addresses = Near.view(quest_id, 'users', {
        pagination: {
          page: 1,
          size: 50,
        },
      });

      const accounts = (addresses || []).map((address) => {
        return {
          account_id: address,
          boxes:
            Near.view(quest_id, 'boxes_for_owner', {
              account_id: address,
              pagination: {
                page: 1,
                size: 40,
              },
            }) || [],
        };
      });

      if (accounts.length === 0)
        return (
          <>
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryText`}
              props={{
                text: 'No boxes have been minted so far',
              }}
            />
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryLinkButton`}
              props={{
                text: 'Mint first Mystery Box',
                href: linkHref({
                  widgetSrc: `${widget_owner_id}/widget/MysteryBox.Manage`,
                  params: {
                    quest_id,
                    page: 'MintBox',
                  },
                }),
              }}
            />
          </>
        );

      return (
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Screens.ListUserBoxes`}
          props={{
            contract: currentQuest,
            accounts,
          }}
        />
      );
    }
    default: {
      return (
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Screens.${page}`}
          props={{
            quest: currentQuest,
          }}
        />
      );
    }
  }
}

console.log('page', page);

return (
  <>
    <Layout
      quest_id={props.quest_id}
      active_home_button={!['Home', 'SignIn'].includes(page)}
    >
      <Page page={page} account_id={account_id} quest_id={props.quest_id} />
    </Layout>
    <Widget
      src={`${widget_owner_id}/widget/Templates.Notification`}
      props={{
        tx_hash: props.transactionHashes,
      }}
    />
  </>
);
