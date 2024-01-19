const RarityButton = styled.button`
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: 0;

  margin: 0;
  padding: 0;

  ${(props) => (props.active ? 'border: 1px solid #8DBFEA;' : 'border: 0;')}

  background: #202f3f;

  display: flex;
  justify-content: center;
  align-items: center;
`;

const InnerRarityButton = styled.div`
  width: 14px;
  height: 14px;
  border-radius: 50%;
  border: 0;
  background: none;

  ${(props) => {
    if (props.rarity === 'rare') return `background: #1EA3AF;`;
    if (props.rarity === 'epic') return `background: #B263C3;`;
    if (props.rarity === 'legendary') return `background: #FBC70F;`;
  }}
`;

const onClick = props.onClick || (() => {});
const active = props.active;
const rarity = props.rarity;
const tooltip = props.tooltip;

return (
  <OverlayTrigger placement="top" overlay={<Tooltip>{tooltip}</Tooltip>}>
    <RarityButton active={active} disabled={active} onClick={onClick}>
      <InnerRarityButton rarity={rarity} />
    </RarityButton>
  </OverlayTrigger>
);
