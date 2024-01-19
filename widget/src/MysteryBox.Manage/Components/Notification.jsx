const Container = styled.div`
  width: 100%;
  height: 100%;

  border: 0;
  border-radius: 10px;

  display: flex;
  align-items: center;
  justify-content: space-evenly;

  padding: 0;
  margin: 0;

  box-shadow: 0px 10px 20px 0px #00000040;

  background: #182432f9;
`;

const IconWrapper = styled.div`
  flex: 1;

  height: 100%;

  display: flex;
  justify-content: center;
  align-items: center;
`;

const Content = styled.div`
  flex: 5;

  height: 80%;

  display: flex;
  flex-direction: column;

  align-items: center;
  justify-content: space-evenly;

  overflow: hidden;
`;
const Action = styled.div`
  flex: 1;

  height: 100%;

  display: flex;
  justify-content: center;
  align-items: center;
`;

const Button = styled.button`
  height: 20px;
  width: 20px;

  border: 0;
  background: none;

  margin: 0;
  padding: 0;
`;

const Title = styled.p`
  margin: 0;
  padding: 0;

  width: 100%;

  font-size: 16px;

  font-family: 'Kodchasan', sans-serif;
  font-weight: 500;
  text-align: center;
  color: #ffffff;

  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
`;

const Subtitle = styled.p`
  margin: 0;
  padding: 0;

  width: 100%;

  font-size: 11px;

  font-family: 'Kodchasan', sans-serif;
  font-weight: 300;
  text-align: center;
  color: ${({ variant }) => {
    switch (variant) {
      case 'success':
        return '#2bccc2';
      case 'error':
        return '#cc2b35';
      default:
        return 'white';
    }
  }};

  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
`;

function Icon({ variant, width, height }) {
  switch (variant) {
    case 'success':
      return (
        <svg
          xmlns="http://www.w3.org/2000/svg"
          x="0px"
          y="0px"
          width={width}
          height={height}
          viewBox="0 0 5.76 5.76"
        >
          <path
            fill="#2bccc2"
            d="M2.88 0.48C1.554 0.48 0.48 1.554 0.48 2.88s1.074 2.4 2.4 2.4 2.4 -1.074 2.4 -2.4S4.206 0.48 2.88 0.48zm-0.48 3.699 -1.13 -1.13 0.339 -0.339L2.4 3.501l1.75 -1.75 0.339 0.339L2.4 4.179z"
          />
        </svg>
      );
    case 'error':
      return (
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width={width}
          height={height}
          viewBox="0 0 5.76 5.76"
        >
          <path
            fill="#cc2b35"
            d="M2.88.48C1.553.48.48 1.553.48 2.88s1.073 2.4 2.4 2.4 2.4-1.073 2.4-2.4S4.207.48 2.88.48zm1.2 3.262-.338.338-.862-.862-.862.862-.338-.338.862-.862-.862-.862.338-.338.862.862.862-.862.338.338-.862.862.862.862z"
          />
        </svg>
      );
    default:
      return <></>;
  }
}

function CloseIcon({ width, height }) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width={width}
      height={height}
      viewBox="0 0 5.2 5.2"
      style={{
        verticalAlign: 'unset',
      }}
    >
      <path
        fill="#638caf"
        d="m4.346 3.928-.42.419a.196.196 0 0 1-.279 0L2.599 3.299 1.552 4.347a.198.198 0 0 1-.28 0l-.42-.419a.198.198 0 0 1 0-.28L1.9 2.6.852 1.552a.198.198 0 0 1 0-.28l.42-.42a.198.198 0 0 1 .28 0L2.599 1.9 3.647.852a.196.196 0 0 1 .279 0l.42.419a.198.198 0 0 1 .002.28L3.298 2.6l1.047 1.048a.198.198 0 0 1 0 .28Z"
      />
    </svg>
  );
}

function Notification({ variant, title, subtitle, onClose }) {
  return (
    <Container>
      <IconWrapper>
        <Icon variant={variant} width={24} height={24} />
      </IconWrapper>
      <Content>
        <Title>{title}</Title>
        {subtitle && <Subtitle variant={variant}>{subtitle}</Subtitle>}
      </Content>
      <Action>
        <Button onClick={onClose || (() => {})}>
          <CloseIcon width={20} height={20} />
        </Button>
      </Action>
    </Container>
  );
}

return <Notification {...props} />;
