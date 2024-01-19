console.log('props', props);

const Button = styled.button`
  background: #638caf;

  border: 0;
  border-radius: 10px;

  width: 240px;
  height: 48px;

  padding: 0;
  margin: 0;

  box-shadow: 0px 20px 40px 0px #00000040;

  background: #182432f0;

  @media (max-width: 678px) {
    height: 36px;
  }
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

  @media (max-width: 678px) {
    font-size: 14px;
  }
`;

return (
  <Button>
    <PrimaryLink href={props.href} target={props.target}>
      {props.text}
    </PrimaryLink>
  </Button>
);
