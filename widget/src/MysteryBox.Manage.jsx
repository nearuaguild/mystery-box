const widget_owner_id = 'evasive-dime.testnet';
const top_contract_id = 'coherent-rail.testnet';

const { logInfo } = VM.require(`${widget_owner_id}/widget/Utils.Logger`);
const rpc_endpoint = 'https://rpc.testnet.near.org';

logInfo("MysteryBox.Manage props", props);

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

  logInfo('result', result);

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

const determinePageAndActiveQuestFromProps = () => {
  if (!account_id) return 'SignIn';

  if (!KnownPages.includes(props.page)) return 'Home';

  if (props.page === 'DeployContract' && props.transactionHashes) {
    const response = fetchTransactionByHash(
      props.transactionHashes,
      account_id
    );

    logInfo('MysteryBox.Manage response', response);

    const result = parseResultFromClaimTransactionResponse(response);

    logInfo('MysteryBox.Manage result', result);

    if (result) {
      return 'Home';
    }
  }

  return props.page;
};

const page = determinePageAndActiveQuestFromProps();

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

  const currentQuest = !isNaN(quest_id) ? quests.find((quest) => quest.quest_id === quest_id) : null;

  logInfo("MysteryBox.Manage quests", { quests, quest_id, currentQuest });

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
                Click below to create a new giveaway and join the Mystery Box community!
                `,
              }}
            />
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryLinkButton`}
              props={{
                text: 'Create new giveaway',
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
            active_quest_id: quest_id, 
          }}
        />
      );
    }
    case 'AddNftReward': {
      const contracts = Near.view(top_contract_id, 'get_trusted_nft_contracts', {});

      logInfo('contracts', contracts);

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

      logInfo("tokens for owner", {tokens, account_id});

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
            quest: currentQuest,
            tokens,
          }}
        />
      );
    }
    case 'ListRewards': {
      /** @todo fetch rarity from backend */

      const fetchRewards = (rarity) => {
        const rewards = Near.view(top_contract_id, 'available_rewards', {
          quest_id,
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
            quest: currentQuest,
            rewards,
          }}
        />
      );
    }
    case 'ListUserBoxes': {
      /** @todo fetch addresses from backend */
      const addresses = Near.view(top_contract_id, 'get_users', {
        quest_id,
        pagination: {
          page: 1,
          size: 50,
        },
      });

      logInfo("addresses", { addresses, quest_id });

      const accounts = (addresses || []).map((address) => {
        return {
          account_id: address,
          boxes:
            Near.view(top_contract_id, 'questboxes_per_owner', {
              account_id: address,
              pagination: {
                page: 1,
                size: 40,
              },
            }) || [],
        };
      });

      logInfo("accounts", addresses, accounts);

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
            quest: currentQuest,
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



const is_active_quest_id_present = props.active_quest_id != undefined && !isNaN(props.active_quest_id);
const is_quest_id_from_props_present = props.quest_id != undefined && !isNaN(props.quest_id);

let quest_id_as_number = 0;

if(is_active_quest_id_present)
{
  quest_id_as_number = parseInt(props.active_quest_id);
}
else if(is_quest_id_from_props_present)
{
  quest_id_as_number = parseInt(props.quest_id);
}

logInfo('page', { page, quest_id_as_number, active_quest_id: props.active_quest_id, props_quest_id: props.quest_id });

return (
  <>
    <Layout
      quest_id={quest_id_as_number}
      active_home_button={!['Home', 'SignIn'].includes(page)}
    >
      <Page page={page} account_id={account_id} quest_id={quest_id_as_number} />
    </Layout>
    <Widget
      src={`${widget_owner_id}/widget/Templates.Notification`}
      props={{
        tx_hash: props.transactionHashes,
      }}
    />
  </>
);
