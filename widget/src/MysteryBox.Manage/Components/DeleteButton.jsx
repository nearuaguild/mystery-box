const WrapperDeleteButton = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
`;

const DeleteButton = styled.button`
  border: 0;
  background: none;

  margin: 0;
  padding: 0;
`;

const onClick = props.onClick || (() => {});

return (
  <WrapperDeleteButton>
    <DeleteButton onClick={onClick}>
      <svg
        width={17}
        height={20}
        viewBox="0 0 17 20"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        {...props}
      >
        <path
          d="M10.885 7.246a.468.468 0 0 0-.468.468v8.853a.469.469 0 0 0 .937 0V7.714a.468.468 0 0 0-.469-.468Zm-5.526 0a.468.468 0 0 0-.468.468v8.853a.469.469 0 0 0 .936 0V7.714a.468.468 0 0 0-.468-.468Z"
          fill="#8DBFEA"
        />
        <path
          d="M1.33 5.954v11.54c0 .682.25 1.323.687 1.782.435.461 1.04.723 1.674.724h8.862a2.306 2.306 0 0 0 1.673-.724c.437-.46.687-1.1.687-1.782V5.954a1.79 1.79 0 0 0-.459-3.518h-2.398V1.85A1.84 1.84 0 0 0 10.201 0H6.042a1.84 1.84 0 0 0-1.855 1.85v.586H1.79a1.79 1.79 0 0 0-.459 3.518Zm11.223 13.11H3.69c-.801 0-1.424-.689-1.424-1.57V5.995h11.71v11.5c0 .88-.623 1.568-1.424 1.568ZM5.124 1.85a.902.902 0 0 1 .918-.913h4.16a.902.902 0 0 1 .918.913v.586H5.123V1.85ZM1.79 3.372h12.665a.843.843 0 1 1 0 1.687H1.79a.843.843 0 1 1 0-1.687Z"
          fill="#8DBFEA"
        />
        <path
          d="M8.122 7.246a.468.468 0 0 0-.469.468v8.853a.468.468 0 0 0 .937 0V7.714a.468.468 0 0 0-.468-.468Z"
          fill="#8DBFEA"
        />
      </svg>
    </DeleteButton>
  </WrapperDeleteButton>
);
