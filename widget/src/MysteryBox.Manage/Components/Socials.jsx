const WrapperSocial = styled.div`
  display: flex;
  width: 120px;
  align-items: center;
  justify-content: space-evenly;
`;

const SocialLink = styled.a`
  cursor: pointer;
  text-align: center;
  margin: 0;
  padding: 0;
`;

const SocialIcon = styled.img`
  width: 16px;
  height: 16px;
  vertical-align: unset;
`;

const SocialButton = styled.button`
  border: 0;
  background: none;

  padding: 0;
`;

return (
  <WrapperSocial>
    <SocialButton>
      <SocialLink href="https://twitter.com/nearuaguild" target="_blank">
        <svg
          width={17.143}
          height={16}
          viewBox="0 0 17.143 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          {...props}
        >
          <path
            d="M13.501 0h2.629l-5.743 6.564 6.756 8.932h-5.29L7.71 10.079l-4.741 5.417H.338l6.143-7.021L0 0h5.424l3.745 4.951L13.501 0Zm-.923 13.922h1.457L4.632 1.491H3.07l9.509 12.432Z"
            fill="#fff"
          />
        </svg>
      </SocialLink>
    </SocialButton>
    <SocialButton>
      <SocialLink
        href="/near/widget/ProfilePage?accountId=nearukraineguild.near"
        target="_blank"
      >
        <svg
          width={16}
          height={16}
          viewBox="0 0 16 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          {...props}
        >
          <path
            d="M14.294 0a1.71 1.71 0 0 0-1.454.812L9.498 5.777a.355.355 0 0 0 .528.465l3.292-2.856a.136.136 0 0 1 .225.1v8.938a.133.133 0 0 1-.236.084L3.354.604A1.713 1.713 0 0 0 2.052 0h-.346A1.705 1.705 0 0 0 0 1.706v12.589a1.705 1.705 0 0 0 3.16.893l3.343-4.964a.356.356 0 0 0-.097-.493.35.35 0 0 0-.428.028l-3.292 2.856a.136.136 0 0 1-.224-.1V3.567a.133.133 0 0 1 .235-.083l9.949 11.912a1.709 1.709 0 0 0 1.302.604h.346A1.702 1.702 0 0 0 16 14.298V1.706A1.705 1.705 0 0 0 14.294 0Z"
            fill="#EBEFEC"
          />
        </svg>
      </SocialLink>
    </SocialButton>
    <SocialButton>
      <SocialLink href="https://t.me/nearprotocolua" target="_blank">
        <svg
          width={18.462}
          height={16}
          viewBox="0 0 18.462 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          {...props}
        >
          <path
            d="M17.548.001a2.046 2.046 0 0 0-.476.047c-.649.138-2.282.774-5.326 2.076C9.403 3.126 1.159 6.782.743 7.004c-.518.276-.746.525-.742.812.004.251.185.447.586.634.186.086 1.342.472 1.998.666.896.266 1.579.406 1.966.406.388 0 .905-.162 1.361-.428.124-.073 1.087-.727 2.138-1.454 2.308-1.596 4.356-2.991 4.966-3.383.406-.262.463-.289.608-.3.139-.01.167-.003.22.056.108.121.109.226.004.388-.181.281-1.42 1.522-3.563 3.571-1.34 1.281-1.979 1.93-2.116 2.15-.286.458-.225.806.218 1.237.212.207.386.33 3.725 2.632a113.584 113.584 0 0 1 1.362.948c.484.357.969.61 1.325.694.098.023.286.037.441.032.234-.006.288-.018.414-.089.351-.196.594-.594.774-1.265.101-.375.516-2.73.935-5.31.569-3.498.943-6.111 1.079-7.545.062-.645-.04-1.089-.289-1.274-.15-.111-.361-.172-.608-.179Z"
            fill="#fff"
          />
        </svg>
      </SocialLink>
    </SocialButton>
  </WrapperSocial>
);
