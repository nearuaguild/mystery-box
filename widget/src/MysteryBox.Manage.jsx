const widget_owner_id = 'denbite.testnet';

const account_id = context.accountId;

const { contract_id, page } = props;

// Import our modules
const { Layout } = VM.require('denbite.testnet/widget/Templates.Layout');

if (!Layout) {
  return <p>Loading modules...</p>;
}

const { href: linkHref } = VM.require('denbite.testnet/widget/core.lib.url');

linkHref || (linkHref = () => {});

const KnownPages = [
  'AddNearReward',
  'AddNftReward',
  'MintBox',
  'ListRewards',
  'ListUserBoxes',
];

if (!account_id) {
  page = 'SignIn';
} else if (!page || !KnownPages.includes(page)) {
  // If no page is specified, we default to the Home page
  page = 'Home';
}

function Page() {
  switch (page) {
    case 'Home': {
      /** @todo fetch a list of contract addresses */

      const contracts = ['dev-1704730152235-47432425806790'];

      if (contracts.length === 0) {
        return (
          <>
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryText`}
              props={{
                text: `Create your first Mystery Box with the button below 👇`,
              }}
            />
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.SubmitButton`}
              props={{
                text: 'Create new contract',
                onClick: () => {},
              }}
            />
          </>
        );
      }

      return (
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Screens.Home`}
          props={{
            defaultContract: contract_id,
            contracts,
          }}
        />
      );
    }
    case 'SignIn': {
      return (
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.PrimaryText`}
          props={{
            text: 'Please sign in with your near wallet to proceed',
          }}
        />
      );
    }
    case 'AddNftReward': {
      const contracts = Near.view(props.contract_id, 'trusted_nft_contracts');

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
              text: 'Please purchase some NFTs in order to distribute them as rewards',
            }}
          />
        );

      return (
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Screens.AddNftReward`}
          props={{
            contract_id,
            tokens,
          }}
        />
      );
    }
    case 'ListRewards': {
      /** @todo fetch rarity from backend */

      const fetchRewards = (rarity) => {
        const rewards = Near.view(props.contract_id, 'rewards', {
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
                  widgetSrc: 'denbite.testnet/widget/MysteryBox.Manage',
                  params: {
                    contract_id: props.contract_id,
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
            contract_id,
            rewards,
          }}
        />
      );
    }
    case 'ListUserBoxes': {
      /** @todo fetch addresses from backend */
      const addresses = Near.view(props.contract_id, 'users', {
        pagination: {
          page: 1,
          size: 50,
        },
      });

      const accounts = (addresses || []).map((address) => {
        return {
          account_id: address,
          boxes:
            Near.view(props.contract_id, 'boxes_for_owner', {
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
                  widgetSrc: 'denbite.testnet/widget/MysteryBox.Manage',
                  params: {
                    contract_id: props.contract_id,
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
            contract_id,
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
            contract_id,
          }}
        />
      );
    }
  }
}

return (
  <Layout page={page} contract_id={contract_id}>
    <Page />
  </Layout>
);
