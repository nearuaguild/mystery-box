console.log('props', props);

const WrapperMenuButton = styled.div`
  flex-basis: 50%;

  display: flex;
  align-items: center;
  justify-content: center;
`;

const MenuButton = styled.button`
  height: 75%;
  width: 90%;

  border: 0;
  border-radius: 10px;

  background: #638caf;

  &:disabled {
    background: #334d64;

    a {
      color: #000000;
    }
  }
`;

const MenuLink = styled.a`
  height: 100%;
  width: 100%;

  display: flex;
  justify-content: center;
  align-items: center;

  font-family: 'Kodchasan', sans-serif;
  font-size: 13px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;

  color: #ffffff;

  &:hover {
    text-decoration: none;
  }
`;

function WrappedMenuLink({ href, text }) {
  if (!href) return <MenuLink>{text}</MenuLink>;

  return <MenuLink href={href}>{text}</MenuLink>;
}

return (
  <WrapperMenuButton>
    <MenuButton disabled={props.disabled}>
      <WrappedMenuLink href={props.href} text={props.text} />
    </MenuButton>
  </WrapperMenuButton>
);
