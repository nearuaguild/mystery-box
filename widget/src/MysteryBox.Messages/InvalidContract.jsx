const CenteredMessage = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100vh;
`;

const Button = styled.button`
  background: "palevioletred";
  color: "white";

  font-size: 1em;
  margin: 1em;
  padding: 0.25em 1em;
  border: 2px solid palevioletred;
  border-radius: 10px;
`;

const TextMessage = styled.p`
  font-size: 24px;
  color: #333;
`;

const CenteredTextComponent = ({ message }) => {
  return (
    <CenteredMessage>
      <TextMessage>{message}</TextMessage>
    </CenteredMessage>
  );
};

return <CenteredTextComponent message={"Invalid contract"} />;
