console.log('props', props);

const Button = styled.button`
  background: #638caf;

  border: 0;
  border-radius: 10px;

  padding: 12px 48px;

  box-shadow: 0px 20px 40px 0px #00000040;

  background: #182432f9;
`;

const PrimaryLink = styled.a`
  height: 100%;
  width: 100%;

  display: flex;
  justify-content: center;
  align-items: center;

  font-family: 'Kodchasan', sans-serif;
  font-size: 16px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;

  color: #ffffff;

  &:hover {
    text-decoration: none;
  }
`;

return (
  <Button>
    <PrimaryLink href={props.href}>{props.text}</PrimaryLink>
  </Button>
);
