const Header = styled.div`
  flex-basis: 20%;
  width: 90%;

  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;

  overflow: hidden;
`;

const Title = styled.p`
  font-family: 'Kodchasan', sans-serif;
  font-size: 24px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  margin: 0;

  flex: 2;

  width: 100%;

  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
`;

const Subtitle = styled.p`
  font-family: 'Kodchasan', sans-serif;
  font-size: 14px;
  font-weight: 400;
  letter-spacing: 0em;
  text-align: center;
  color: rgba(43, 204, 194, 1);
  margin: 0;

  flex: 1;

  width: 100%;

  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
`;

return (
  <Header>
    <Title>{props.title}</Title>
    <Subtitle>{props.subtitle}</Subtitle>
  </Header>
);
