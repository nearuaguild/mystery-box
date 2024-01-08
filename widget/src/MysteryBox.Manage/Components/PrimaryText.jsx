const PrimaryText = styled.p`
  @media (min-width: 512px) {
    font-size: 28px;
    width: 80%;
  }

  width: 90%;
  font-size: 20px;

  font-family: 'Kodchasan', sans-serif;
  font-weight: 700;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;
`;

return <PrimaryText>{props.text}</PrimaryText>;
