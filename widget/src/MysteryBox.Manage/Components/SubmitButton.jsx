const Button = styled.button`
  background: #638caf;

  border: 0;
  border-radius: 10px;

  padding: 4px 64px;

  font-family: 'Kodchasan', sans-serif;
  font-size: 20px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;

  box-shadow: 0px 20px 40px 0px #00000040;

  &:disabled {
    background: #466988;
  }
`;

return (
  <Button onClick={props.onClick} disabled={props.disabled || false}>
    {props.text}
  </Button>
);
