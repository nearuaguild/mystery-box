const { href: linkHref } = VM.require('denbite.testnet/widget/core.lib.url');

linkHref || (linkHref = () => {});

const HomeButton = styled.button`
  border: 0;
  background: none;
`;

if (!props.active) {
  return (
    <HomeButton disabled>
      <svg
        width={66.923}
        height={20}
        viewBox="0 0 66.923 20"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        {...props}
      >
        <path
          d="M28.773 16.277c-.246 0-.443-.065-.591-.197-.148-.131-.222-.312-.222-.542V8.153c0-.23.074-.41.222-.542.148-.131.345-.197.591-.197s.443.069.591.209a.656.656 0 0 1 .234.529v2.942h4.418v-2.94a.67.67 0 0 1 .222-.529c.156-.139.357-.209.603-.209s.443.065.591.197c.148.131.222.312.222.542v7.385c0 .23-.074.41-.222.542-.148.131-.345.197-.591.197s-.447-.065-.603-.197a.712.712 0 0 1-.222-.542v-3.175h-4.418v3.173a.692.692 0 0 1-.234.542c-.148.131-.345.197-.591.197Zm12.742 0c-.96 0-1.788-.185-2.486-.554a3.855 3.855 0 0 1-1.575-1.563c-.362-.673-.542-1.444-.542-2.314 0-.869.181-1.637.542-2.302a3.758 3.758 0 0 1 1.575-1.563c.698-.377 1.526-.566 2.486-.566.96 0 1.785.188 2.474.566a3.742 3.742 0 0 1 1.588 1.563c.362.665.542 1.432.542 2.302 0 .869-.181 1.641-.542 2.314a3.841 3.841 0 0 1-1.588 1.563c-.689.369-1.514.554-2.474.554Zm0-1.292c.935 0 1.662-.279 2.178-.837.517-.566.775-1.333.775-2.302 0-.977-.271-1.744-.812-2.302-.533-.558-1.288-.837-2.265-.837-.895 0-1.592.287-2.092.862-.492.566-.738 1.325-.738 2.277 0 .968.258 1.735.775 2.302.517.558 1.243.837 2.178.837Zm6.356 1.292c-.246 0-.443-.065-.591-.197-.148-.131-.222-.312-.222-.542v-5.071c0-.952.271-1.698.812-2.24.542-.542 1.288-.812 2.24-.812.46 0 .891.115 1.292.345a3.092 3.092 0 0 1 1.058.96 2.982 2.982 0 0 1 1.034-.972 2.754 2.754 0 0 1 1.329-.332c.952 0 1.698.271 2.24.812.542.542.812 1.288.812 2.24v5.07a.693.693 0 0 1-.234.542c-.148.131-.345.197-.591.197s-.443-.065-.591-.197c-.148-.131-.222-.312-.222-.542v-5.071c0-.623-.115-1.071-.345-1.342-.229-.279-.608-.418-1.132-.418-.475 0-.845.152-1.108.455-.254.304-.382.738-.382 1.305v5.071c0 .23-.074.41-.222.542-.148.131-.345.197-.591.197s-.443-.065-.591-.197c-.139-.131-.209-.312-.209-.542v-5.071c0-.55-.131-.981-.394-1.292-.262-.312-.628-.468-1.095-.468-.525 0-.902.139-1.132.418-.229.271-.345.718-.345 1.342v5.071a.693.693 0 0 1-.234.542c-.148.131-.345.197-.591.197Zm13.357-.123c-.624 0-1.108-.164-1.452-.492-.345-.328-.517-.788-.517-1.378V9.361c0-.575.164-1.022.492-1.342.336-.32.8-.48 1.391-.48h3.828c.246 0 .435.058.566.172.131.107.197.262.197.468a.575.575 0 0 1-.209.468c-.132.107-.316.16-.554.16h-3.545a.51.51 0 0 0-.394.16c-.091.107-.135.254-.135.443v1.711h3.138c.246 0 .435.058.566.172.131.107.197.262.197.468a.575.575 0 0 1-.209.468c-.132.107-.316.16-.554.16h-3.138v1.895c0 .188.045.336.135.443a.51.51 0 0 0 .394.16h3.815c.246 0 .435.058.566.172.131.107.197.262.197.468a.575.575 0 0 1-.209.468c-.132.107-.316.16-.554.16h-4.012Z"
          fill="#fff"
        />
        <path
          fillRule="evenodd"
          clipRule="evenodd"
          d="M8.023.615a3.481 3.481 0 0 1 3.954 0l6.5 4.478A3.531 3.531 0 0 1 20 8.001v8.476C20 18.423 18.433 20 16.5 20h-13C1.567 20 0 18.423 0 16.477V8.001a3.531 3.531 0 0 1 1.523-2.908l6.5-4.478ZM7.5 15.469a.502.502 0 0 0-.5.504c0 .278.224.503.5.503h5c.276 0 .5-.225.5-.503a.502.502 0 0 0-.5-.504h-5Z"
          fill="#fff"
        />
      </svg>
    </HomeButton>
  );
}

return (
  <HomeButton>
    <Link
      to={linkHref({
        widgetSrc: 'denbite.testnet/widget/MysteryBox.Manage',
        params: {
          contract_id: props.contract_id,
        },
      })}
    >
      <svg
        width={66.923}
        height={20}
        viewBox="0 0 66.923 20"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        {...props}
      >
        <path
          d="M29.318 16.277a.449.449 0 0 1-.332-.123.527.527 0 0 1-.123-.369V7.908c0-.156.041-.275.123-.357a.427.427 0 0 1 .332-.135c.148 0 .262.045.345.135.082.082.123.201.123.357v3.434h5.403V7.908c0-.156.041-.275.123-.357a.443.443 0 0 1 .345-.135c.139 0 .25.045.332.135.082.082.123.201.123.357v7.877a.527.527 0 0 1-.123.369.449.449 0 0 1-.332.123.492.492 0 0 1-.345-.123.527.527 0 0 1-.123-.369v-3.668h-5.403v3.668a.527.527 0 0 1-.123.369.466.466 0 0 1-.345.123Zm12.813 0c-.828 0-1.575-.181-2.24-.542a4.062 4.062 0 0 1-1.575-1.563c-.377-.673-.566-1.448-.566-2.326 0-.886.188-1.662.566-2.326a3.976 3.976 0 0 1 1.575-1.551c.665-.369 1.412-.554 2.24-.554s1.575.185 2.24.554a3.894 3.894 0 0 1 1.563 1.551c.385.665.578 1.44.578 2.326 0 .878-.192 1.654-.578 2.326a3.977 3.977 0 0 1-1.563 1.563c-.665.362-1.412.542-2.24.542Zm0-.788c.665 0 1.255-.144 1.772-.431a3.044 3.044 0 0 0 1.231-1.243c.304-.55.455-1.206.455-1.969 0-.772-.156-1.432-.468-1.982a3.004 3.004 0 0 0-1.255-1.243 3.692 3.692 0 0 0-1.76-.418c-.656 0-1.247.144-1.772.431a3.04 3.04 0 0 0-1.218 1.255c-.295.55-.443 1.202-.443 1.957 0 .763.148 1.419.443 1.969a3.147 3.147 0 0 0 1.231 1.243c.525.287 1.12.431 1.785.431Zm6.115.788a.449.449 0 0 1-.332-.123.527.527 0 0 1-.123-.369v-5.563c0-.869.246-1.555.738-2.055.492-.5 1.169-.751 2.031-.751.5 0 .96.131 1.378.394.426.254.746.595.96 1.022a2.476 2.476 0 0 1 .935-1.034 2.648 2.648 0 0 1 1.403-.382c.862 0 1.538.25 2.031.751.492.492.738 1.177.738 2.055v5.563a.499.499 0 0 1-.135.369.449.449 0 0 1-.332.123.466.466 0 0 1-.345-.123.564.564 0 0 1-.111-.369v-5.563c0-.648-.148-1.145-.443-1.489-.288-.345-.763-.517-1.428-.517-.583 0-1.038.181-1.366.542-.328.353-.492.841-.492 1.465v5.563a.532.532 0 0 1-.123.369.449.449 0 0 1-.332.123.449.449 0 0 1-.332-.123.564.564 0 0 1-.111-.369v-5.563c0-.608-.172-1.092-.517-1.452-.345-.369-.796-.554-1.354-.554-.665 0-1.145.172-1.44.517-.288.345-.431.841-.431 1.489v5.563a.532.532 0 0 1-.123.369.466.466 0 0 1-.345.123Zm13.262-.123c-.5 0-.89-.131-1.169-.394-.278-.262-.418-.632-.418-1.108V9.015c0-.459.135-.821.406-1.083.279-.262.665-.394 1.157-.394h3.815c.32 0 .48.131.48.394a.329.329 0 0 1-.123.271c-.082.065-.201.098-.357.098h-3.803a.628.628 0 0 0-.48.197c-.115.131-.172.304-.172.517v2.351h3.52c.32 0 .48.131.48.394a.328.328 0 0 1-.135.283.543.543 0 0 1-.345.098h-3.52v2.535c0 .213.058.385.172.517a.629.629 0 0 0 .48.197h4.074c.32 0 .48.127.48.382a.328.328 0 0 1-.135.283.543.543 0 0 1-.345.098h-4.062ZM12.5 15.54c.277 0 .5.222.5.495a.498.498 0 0 1-.5.495h-5a.498.498 0 0 1-.5-.495c0-.274.224-.495.5-.495h5Z"
          fill="#fff"
        />
        <path
          fillRule="evenodd"
          clipRule="evenodd"
          d="M7.831.746a3.525 3.525 0 0 1 4.338 0l6.5 5.086A3.458 3.458 0 0 1 20 8.555v7.976C20 18.446 18.433 20 16.5 20h-13C1.567 20 0 18.446 0 16.531V8.555a3.462 3.462 0 0 1 1.331-2.723l6.5-5.086Zm3.718.778a2.518 2.518 0 0 0-3.098 0l-6.5 5.086A2.47 2.47 0 0 0 1 8.555v7.976a2.489 2.489 0 0 0 2.5 2.478h13a2.49 2.49 0 0 0 2.5-2.478V8.555c0-.758-.351-1.475-.951-1.945l-6.5-5.086Z"
          fill="#fff"
        />
      </svg>
    </Link>
  </HomeButton>
);
