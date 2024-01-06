console.log('ClaimAnimation.props', props);

const font = fetch(
  'https://fonts.googleapis.com/css2?family=Lilita+One:wght@400&display=swap'
).body;

const font2 = fetch(
  'https://fonts.googleapis.com/css2?family=Kodchasan:wght@700&display=swap'
).body;

if (!font || !font2) {
  return <></>;
}

const Wrapper = styled.div`
  position: fixed;
  top: 64px;
  bottom: 0;
  left: 0;
  right: 0;
  width: 100%;

  ${font}
  ${font2}
`;

const ContentWrapper = styled.div`
  ${(props) => {
    if (!props.landscape || !props.portrait)
      return `background-color: rgba(25, 25, 25, 0.25);`;

    return `
      @media (orientation: landscape) {
        background-image: url("${props.landscape}");
      }
    
      @media (orientation: portrait) {
        background-image: url("${props.portrait}");
      }
      `;
  }}

  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background-repeat: no-repeat;
  background-position: center;
  background-size: cover;
`;

const RewardBoundary = styled.div`
  width: 240px;
  height: 240px;
  border-radius: 30px;

  ${(props) => {
    if (props.rarity === 'rare')
      return `
            border: 4px solid #4e70c9;
            box-shadow: 0 0 40px rgba(78, 112, 201, 0.4),
            inset 0 0 40px rgba(78, 112, 201, 0.4);
    `;

    if (props.rarity === 'epic')
      return `
            border: 4px solid #8357AD;
            box-shadow: 0 0 40px rgba(131, 87, 173, 0.4),
            inset 0 0 40px rgba(131, 87, 173, 0.4);
    `;

    if (props.rarity === 'legendary')
      return `
            border: 4px solid #D99B38;
            box-shadow: 0 0 40px rgba(217, 155, 56, 0.4),
            inset 0 0 40px rgba(217, 155, 56, 0.4);
    `;
  }}

  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;
`;

const NearRewardAmount = styled.div`
  padding: 0;
  margin: 0;

  color: #fff;
  text-align: center;
  font-family: Lilita One;
  font-size: 96px;
  line-height: 1;
  font-style: normal;
  font-weight: 400;
`;

const NonFungibleTokenRewardImage = styled.div`
  height: 88%;
  width: 88%;

  border-radius: 20px;

  ${(props) => {
    if (!props.src) return `background-color: rgba(255, 255, 255, 0.2);`;

    return `
    background-image: url(${props.src});
    background-size: cover;
    background-position: center;
    `;
  }}
`;

const PrimaryText = styled.div`
  @media (min-width: 512px) {
    font-size: 24px;
    line-height: 32px;
  }

  font-size: 18px;
  line-height: 24px;

  font-family: 'Kodchasan', sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;
`;

const WhiteButton = styled.div`
  position: relative;
  cursor: pointer;

  text-align: none;
  text-decoration: none;

  @media (min-width: 512px) {
    font-size: 18px;
    width: 160px;
  }

  font-size: 16px;
  width: 120px;

  line-height: 1;
  padding: 0.5em 2em;

  font-family: 'Kodchasan', sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;

  background: none;
  border-radius: 100px;
  border: 3px solid rgb(255, 255, 255);
  box-shadow: 0 0 25px rgba(255, 255, 255, 0.3),
    inset 0 0 25px rgba(255, 255, 255, 0.3);
`;

const NearRewardIcon = (props) => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width={props.width}
      height={props.height}
      viewBox="0 0 39.434 10"
      fill="none"
    >
      <mask
        id="a"
        style={{ maskType: 'luminance' }}
        maskUnits="userSpaceOnUse"
        x="0"
        y="0"
        width="209"
        height="53"
      >
        <path d="M39.434.048H0V9.97h39.434V.048Z" fill="#fff" />
      </mask>
      <g mask="url(#a)" fill="#fff">
        <path d="M16.34 1.583c-.728 0-1.257.171-1.707.567l-.794.685c-.066.052-.199.092-.292.014-.092-.079-.106-.185-.027-.291l.423-.633c.066-.092.014-.212-.106-.212h-1.018a.208.208 0 0 0-.212.211v6.168a.21.21 0 0 0 .212.212h1.058c.12 0 .212-.092.212-.211V4.614c0-1.595 1.336-1.845 1.839-1.845 1.072 0 1.454.764 1.454 1.345v3.981a.21.21 0 0 0 .212.212h1.058a.209.209 0 0 0 .212-.211V3.983c0-1.476-.966-2.399-2.526-2.399v-.002Zm6.839-.028c-2.051 0-3.36 1.252-3.36 2.952v.935c0 1.793 1.309 3.019 3.36 3.019 1.812 0 3.082-.935 3.214-2.201.013-.132-.079-.223-.212-.223H25.15a.199.199 0 0 0-.199.144c-.133.422-.753 1.054-1.773 1.054-1.019 0-1.972-.739-1.958-1.793l.013-1.174c.014-.883.939-1.489 1.945-1.489.913 0 1.798.514 1.891 1.357a.195.195 0 0 1-.157.205l-2.966.574a.269.269 0 0 0-.212.264v.014c0 .119.12.223.292.223h4.258a.212.212 0 0 0 .212-.212v-.83c0-1.568-1.362-2.821-3.32-2.821l.003.002Zm7.38.001c-1.653 0-3.082.962-3.082 2.228 0 .106.093.185.212.185h1.072c.106 0 .185-.079.199-.185.106-.579.807-1.002 1.561-1.002.9 0 1.508.554 1.508 1.502v1.147c0 1.174-.873 1.766-1.958 1.766-.846 0-1.336-.315-1.336-.831 0-.448.238-.831 1.217-1.054l1.415-.383c.145-.04.199-.158.172-.291-.014-.106-.132-.158-.238-.158h-1.468c-1.244 0-2.499.791-2.499 1.951v.185c0 1.185 1.124 1.805 2.408 1.805.821 0 1.522-.316 1.958-.685l.649-.554c.106-.092.212-.092.305 0 .079.079.052.198-.014.291l-.396.619c-.066.092-.013.212.106.212h.952c.12 0 .212-.092.212-.211V4.101c0-1.529-1.097-2.543-2.949-2.543h-.007Zm8.663.157h-1.481c-.516 0-1.018.316-1.375.62l-.581.5c-.066.052-.185.092-.265.027-.093-.065-.133-.198-.052-.304l.423-.633c.066-.092.014-.212-.106-.212h-.992a.208.208 0 0 0-.212.211v6.169a.21.21 0 0 0 .212.212h1.085c.12 0 .212-.092.212-.212V4.928c0-1.357.556-1.964 1.759-1.964h1.375c.12 0 .212-.092.212-.212v-.831a.21.21 0 0 0-.212-.212l-.002.003ZM8.898.048a1.066 1.066 0 0 0-.905.504L5.912 3.63a.219.219 0 0 0 .062.306.219.219 0 0 0 .266-.017l2.049-1.771a.085.085 0 0 1 .118.007.085.085 0 0 1 .022.055v5.543a.082.082 0 0 1-.147.052L2.088.422a1.068 1.068 0 0 0-.811-.374h-.215A1.06 1.06 0 0 0 0 1.105v7.808a1.06 1.06 0 0 0 1.062 1.058 1.066 1.066 0 0 0 .905-.504l2.081-3.079a.22.22 0 0 0-.06-.306.218.218 0 0 0-.266.017L1.673 7.87a.085.085 0 0 1-.118-.007.085.085 0 0 1-.022-.055V2.261a.083.083 0 0 1 .147-.052l6.193 7.389a1.065 1.065 0 0 0 .811.374H8.9c.586 0 1.062-.472 1.062-1.056v-7.81A1.06 1.06 0 0 0 8.9.048Z" />
      </g>
    </svg>
  );
};

const onClick = () => {
  console.log('ClaimAnimation', 'Clicked button to go back');

  return props.onBack();
};

const LoadingReward = ({ width, height }) => {
  const source = `<div id="main"></div>
  <style>
  html {
    width: ${width}px;
    height: ${height}px;
  }

  body {
    padding: 0;
    margin: 0;
  }
  </style>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/lottie-web/5.12.2/lottie.min.js"></script>
  <script>
  const element = document.getElementById('main');

  const animation = {
    v: "4.8.0",
    meta: { g: "LottieFiles AE 3.4.5", a: "", k: "", d: "", tc: "" },
    fr: 30,
    ip: 0,
    op: 151,
    w: 800,
    h: 800,
    nm: "Lottie_Main_Comp 2",
    ddd: 0,
    assets: [
      {
        id: "image_0",
        w: 636,
        h: 633,
        u: "",
        p: "data:image/png;base64,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",
        e: 1,
      },
      {
        id: "comp_0",
        layers: [
          {
            ddd: 0,
            ind: 1,
            ty: 2,
            nm: "path2723.ai",
            cl: "ai",
            refId: "image_0",
            sr: 1,
            ks: {
              o: { a: 0, k: 100, ix: 11 },
              r: { a: 0, k: 0, ix: 10 },
              p: { a: 0, k: [400, 400, 0], ix: 2 },
              a: { a: 0, k: [318, 316.5, 0], ix: 1 },
              s: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [1, 1, -0.814] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 2.444] },
                    t: 0,
                    s: [24.214, 24.214, 100],
                  },
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, -0.389] },
                    o: { x: [0.167, 0.167, 0.167], y: [0, 0, 2.019] },
                    t: 71,
                    s: [34.591, 34.591, 100],
                  },
                  { t: 150, s: [24.214, 24.214, 100] },
                ],
                ix: 6,
              },
            },
            ao: 0,
            ip: 0,
            op: 151,
            st: 0,
            bm: 0,
          },
          {
            ddd: 0,
            ind: 3,
            ty: 4,
            nm: "形状图层 10",
            sr: 1,
            ks: {
              o: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833], y: [0.833] },
                    o: { x: [0.167], y: [0.167] },
                    t: 2,
                    s: [100],
                  },
                  { t: 62, s: [0] },
                ],
                ix: 11,
              },
              r: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833], y: [0.833] },
                    o: { x: [0.167], y: [0.167] },
                    t: -304,
                    s: [0],
                  },
                  {
                    i: { x: [0.833], y: [0.833] },
                    o: { x: [0.167], y: [0.167] },
                    t: -153,
                    s: [120],
                  },
                  {
                    i: { x: [0.833], y: [0.833] },
                    o: { x: [0.167], y: [0.167] },
                    t: 0,
                    s: [360],
                  },
                  { t: 151, s: [480] },
                ],
                ix: 10,
              },
              p: { a: 0, k: [400, 400, 0], ix: 2 },
              a: { a: 0, k: [62.471, -110.018, 0], ix: 1 },
              s: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: -304,
                    s: [100, 100, 100],
                  },
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: -153,
                    s: [124, 124, 100],
                  },
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: 0,
                    s: [148, 148, 100],
                  },
                  { t: 151, s: [166, 166, 100] },
                ],
                ix: 6,
              },
            },
            ao: 0,
            shapes: [
              {
                ty: "gr",
                it: [
                  {
                    d: 1,
                    ty: "el",
                    s: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                          o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                          t: -304,
                          s: [40, 40],
                        },
                        {
                          i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                          o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                          t: -153,
                          s: [24, 24],
                        },
                        {
                          i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                          o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                          t: 0,
                          s: [16, 16],
                        },
                        { t: 151, s: [0, 0] },
                      ],
                      ix: 2,
                    },
                    p: { a: 0, k: [0, 0], ix: 3 },
                    nm: "椭圆路径 1",
                    mn: "ADBE Vector Shape - Ellipse",
                    hd: false,
                  },
                  {
                    ty: "fl",
                    c: {
                      a: 0,
                      k: [0.60282599926, 0.146082267165, 0.631372570992, 1],
                      ix: 4,
                    },
                    o: { a: 0, k: 100, ix: 5 },
                    r: 1,
                    bm: 0,
                    nm: "填充 1",
                    mn: "ADBE Vector Graphic - Fill",
                    hd: false,
                  },
                  {
                    ty: "gr",
                    it: [
                      {
                        d: 1,
                        ty: "el",
                        s: {
                          a: 1,
                          k: [
                            {
                              i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                              o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                              t: -304,
                              s: [40, 40],
                            },
                            {
                              i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                              o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                              t: -153,
                              s: [24, 24],
                            },
                            {
                              i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                              o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                              t: 2,
                              s: [16, 16],
                            },
                            { t: 151, s: [0, 0] },
                          ],
                          ix: 2,
                        },
                        p: { a: 0, k: [0, 0], ix: 3 },
                        nm: "椭圆路径 1",
                        mn: "ADBE Vector Shape - Ellipse",
                        hd: false,
                      },
                      {
                        ty: "fl",
                        c: {
                          a: 0,
                          k: [0.670000016689, 0.52999997139, 0.870000004768, 1],
                          ix: 4,
                        },
                        o: { a: 0, k: 100, ix: 5 },
                        r: 1,
                        bm: 0,
                        nm: "填充 1",
                        mn: "ADBE Vector Graphic - Fill",
                        hd: false,
                      },
                      {
                        ty: "tr",
                        p: { a: 0, k: [472.605, -2.23], ix: 2 },
                        a: { a: 0, k: [0, 0], ix: 1 },
                        s: { a: 0, k: [100, 100], ix: 3 },
                        r: { a: 0, k: 0, ix: 6 },
                        o: { a: 0, k: 100, ix: 7 },
                        sk: { a: 0, k: 0, ix: 4 },
                        sa: { a: 0, k: 0, ix: 5 },
                        nm: "Transform",
                      },
                    ],
                    nm: "椭圆 1",
                    np: 2,
                    cix: 2,
                    bm: 0,
                    ix: 3,
                    mn: "ADBE Vector Group",
                    hd: false,
                  },
                  {
                    ty: "tr",
                    p: { a: 0, k: [-173.832, -108.902], ix: 2 },
                    a: { a: 0, k: [0, 0], ix: 1 },
                    s: { a: 0, k: [100, 100], ix: 3 },
                    r: { a: 0, k: 0, ix: 6 },
                    o: { a: 0, k: 100, ix: 7 },
                    sk: { a: 0, k: 0, ix: 4 },
                    sa: { a: 0, k: 0, ix: 5 },
                    nm: "Transform",
                  },
                ],
                nm: "椭圆 1",
                np: 3,
                cix: 2,
                bm: 0,
                ix: 1,
                mn: "ADBE Vector Group",
                hd: false,
              },
            ],
            ip: -304,
            op: 636,
            st: -304,
            bm: 0,
          },
          {
            ddd: 0,
            ind: 4,
            ty: 4,
            nm: "形状图层 9",
            sr: 1,
            ks: {
              o: { a: 0, k: 100, ix: 11 },
              r: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833], y: [0.833] },
                    o: { x: [0.167], y: [0.167] },
                    t: -151,
                    s: [0],
                  },
                  {
                    i: { x: [0.833], y: [0.833] },
                    o: { x: [0.167], y: [0.167] },
                    t: 0,
                    s: [240],
                  },
                  { t: 151, s: [360] },
                ],
                ix: 10,
              },
              p: { a: 0, k: [400, 400, 0], ix: 2 },
              a: { a: 0, k: [62.471, -110.018, 0], ix: 1 },
              s: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: -151,
                    s: [100, 100, 100],
                  },
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: 0,
                    s: [124, 124, 100],
                  },
                  { t: 151, s: [148, 148, 100] },
                ],
                ix: 6,
              },
            },
            ao: 0,
            shapes: [
              {
                ty: "gr",
                it: [
                  {
                    d: 1,
                    ty: "el",
                    s: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                          o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                          t: -151,
                          s: [40, 40],
                        },
                        {
                          i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                          o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                          t: 0,
                          s: [24, 24],
                        },
                        { t: 151, s: [16, 16] },
                      ],
                      ix: 2,
                    },
                    p: { a: 0, k: [0, 0], ix: 3 },
                    nm: "椭圆路径 1",
                    mn: "ADBE Vector Shape - Ellipse",
                    hd: false,
                  },
                  {
                    ty: "fl",
                    c: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833], y: [1] },
                          o: { x: [0.167], y: [0] },
                          t: 0,
                          s: [0.769999980927, 0.670000016689, 0.889999985695, 1],
                        },
                        {
                          t: 151,
                          s: [0.670000016689, 0.52999997139, 0.870000004768, 1],
                        },
                      ],
                      ix: 4,
                    },
                    o: { a: 0, k: 100, ix: 5 },
                    r: 1,
                    bm: 0,
                    nm: "填充 1",
                    mn: "ADBE Vector Graphic - Fill",
                    hd: false,
                  },
                  {
                    ty: "gr",
                    it: [
                      {
                        d: 1,
                        ty: "el",
                        s: {
                          a: 1,
                          k: [
                            {
                              i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                              o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                              t: -151,
                              s: [40, 40],
                            },
                            {
                              i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                              o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                              t: 0,
                              s: [24, 24],
                            },
                            { t: 151, s: [16, 16] },
                          ],
                          ix: 2,
                        },
                        p: { a: 0, k: [0, 0], ix: 3 },
                        nm: "椭圆路径 1",
                        mn: "ADBE Vector Shape - Ellipse",
                        hd: false,
                      },
                      {
                        ty: "fl",
                        c: {
                          a: 1,
                          k: [
                            {
                              i: { x: [0.833], y: [1] },
                              o: { x: [0.167], y: [0] },
                              t: 0,
                              s: [
                                0.769999980927, 0.670000016689, 0.889999985695, 1,
                              ],
                            },
                            {
                              t: 151,
                              s: [
                                0.670000016689, 0.52999997139, 0.870000004768, 1,
                              ],
                            },
                          ],
                          ix: 4,
                        },
                        o: { a: 0, k: 100, ix: 5 },
                        r: 1,
                        bm: 0,
                        nm: "填充 1",
                        mn: "ADBE Vector Graphic - Fill",
                        hd: false,
                      },
                      {
                        ty: "tr",
                        p: { a: 0, k: [472.605, -2.23], ix: 2 },
                        a: { a: 0, k: [0, 0], ix: 1 },
                        s: { a: 0, k: [100, 100], ix: 3 },
                        r: { a: 0, k: 0, ix: 6 },
                        o: { a: 0, k: 100, ix: 7 },
                        sk: { a: 0, k: 0, ix: 4 },
                        sa: { a: 0, k: 0, ix: 5 },
                        nm: "Transform",
                      },
                    ],
                    nm: "椭圆 1",
                    np: 2,
                    cix: 2,
                    bm: 0,
                    ix: 3,
                    mn: "ADBE Vector Group",
                    hd: false,
                  },
                  {
                    ty: "tr",
                    p: { a: 0, k: [-173.832, -108.902], ix: 2 },
                    a: { a: 0, k: [0, 0], ix: 1 },
                    s: { a: 0, k: [100, 100], ix: 3 },
                    r: { a: 0, k: 0, ix: 6 },
                    o: { a: 0, k: 100, ix: 7 },
                    sk: { a: 0, k: 0, ix: 4 },
                    sa: { a: 0, k: 0, ix: 5 },
                    nm: "Transform",
                  },
                ],
                nm: "椭圆 1",
                np: 3,
                cix: 2,
                bm: 0,
                ix: 1,
                mn: "ADBE Vector Group",
                hd: false,
              },
            ],
            ip: -151,
            op: 789,
            st: -151,
            bm: 0,
          },
          {
            ddd: 0,
            ind: 5,
            ty: 4,
            nm: "形状图层 4",
            sr: 1,
            ks: {
              o: { a: 0, k: 100, ix: 11 },
              r: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833], y: [0.833] },
                    o: { x: [0.167], y: [0.167] },
                    t: 0,
                    s: [120],
                  },
                  { t: 151, s: [240] },
                ],
                ix: 10,
              },
              p: { a: 0, k: [400, 400, 0], ix: 2 },
              a: { a: 0, k: [62.471, -110.018, 0], ix: 1 },
              s: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: 0,
                    s: [100, 100, 100],
                  },
                  { t: 151, s: [124, 124, 100] },
                ],
                ix: 6,
              },
            },
            ao: 0,
            shapes: [
              {
                ty: "gr",
                it: [
                  {
                    d: 1,
                    ty: "el",
                    s: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                          o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                          t: 0,
                          s: [40, 40],
                        },
                        { t: 151, s: [24, 24] },
                      ],
                      ix: 2,
                    },
                    p: { a: 0, k: [0, 0], ix: 3 },
                    nm: "椭圆路径 1",
                    mn: "ADBE Vector Shape - Ellipse",
                    hd: false,
                  },
                  {
                    ty: "fl",
                    c: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833], y: [1] },
                          o: { x: [0.167], y: [0] },
                          t: 0,
                          s: [1, 1, 1, 1],
                        },
                        {
                          t: 149,
                          s: [0.769999980927, 0.680000007153, 0.889999985695, 1],
                        },
                      ],
                      ix: 4,
                    },
                    o: { a: 0, k: 100, ix: 5 },
                    r: 1,
                    bm: 0,
                    nm: "填充 1",
                    mn: "ADBE Vector Graphic - Fill",
                    hd: false,
                  },
                  {
                    ty: "gr",
                    it: [
                      {
                        d: 1,
                        ty: "el",
                        s: {
                          a: 1,
                          k: [
                            {
                              i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                              o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                              t: 0,
                              s: [40, 40],
                            },
                            { t: 151, s: [24, 24] },
                          ],
                          ix: 2,
                        },
                        p: { a: 0, k: [0, 0], ix: 3 },
                        nm: "椭圆路径 1",
                        mn: "ADBE Vector Shape - Ellipse",
                        hd: false,
                      },
                      {
                        ty: "fl",
                        c: {
                          a: 1,
                          k: [
                            {
                              i: { x: [0.833], y: [1] },
                              o: { x: [0.167], y: [0] },
                              t: 0,
                              s: [1, 1, 1, 1],
                            },
                            {
                              t: 149,
                              s: [
                                0.769999980927, 0.680000007153, 0.889999985695, 1,
                              ],
                            },
                          ],
                          ix: 4,
                        },
                        o: { a: 0, k: 100, ix: 5 },
                        r: 1,
                        bm: 0,
                        nm: "填充 1",
                        mn: "ADBE Vector Graphic - Fill",
                        hd: false,
                      },
                      {
                        ty: "tr",
                        p: { a: 0, k: [472.605, -2.23], ix: 2 },
                        a: { a: 0, k: [0, 0], ix: 1 },
                        s: { a: 0, k: [100, 100], ix: 3 },
                        r: { a: 0, k: 0, ix: 6 },
                        o: { a: 0, k: 100, ix: 7 },
                        sk: { a: 0, k: 0, ix: 4 },
                        sa: { a: 0, k: 0, ix: 5 },
                        nm: "Transform",
                      },
                    ],
                    nm: "椭圆 1",
                    np: 2,
                    cix: 2,
                    bm: 0,
                    ix: 3,
                    mn: "ADBE Vector Group",
                    hd: false,
                  },
                  {
                    ty: "tr",
                    p: { a: 0, k: [-173.832, -108.902], ix: 2 },
                    a: { a: 0, k: [0, 0], ix: 1 },
                    s: { a: 0, k: [100, 100], ix: 3 },
                    r: { a: 0, k: 0, ix: 6 },
                    o: { a: 0, k: 100, ix: 7 },
                    sk: { a: 0, k: 0, ix: 4 },
                    sa: { a: 0, k: 0, ix: 5 },
                    nm: "Transform",
                  },
                ],
                nm: "椭圆 1",
                np: 3,
                cix: 2,
                bm: 0,
                ix: 1,
                mn: "ADBE Vector Group",
                hd: false,
              },
            ],
            ip: -11,
            op: 940,
            st: 0,
            bm: 0,
          },
          {
            ddd: 0,
            ind: 6,
            ty: 4,
            nm: "形状图层 11",
            sr: 1,
            ks: {
              o: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833], y: [0.833] },
                    o: { x: [0.167], y: [0.167] },
                    t: 0,
                    s: [0],
                  },
                  { t: 57, s: [100] },
                ],
                ix: 11,
              },
              r: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833], y: [0.833] },
                    o: { x: [0.167], y: [0.167] },
                    t: 0,
                    s: [0],
                  },
                  {
                    i: { x: [0.833], y: [1] },
                    o: { x: [0.167], y: [0] },
                    t: 151,
                    s: [120],
                  },
                  { t: 302, s: [120] },
                ],
                ix: 10,
              },
              p: { a: 0, k: [400, 400, 0], ix: 2 },
              a: { a: 0, k: [62.471, -110.018, 0], ix: 1 },
              s: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: 0,
                    s: [76, 76, 100],
                  },
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: 151,
                    s: [100, 100, 100],
                  },
                  { t: 302, s: [124, 124, 100] },
                ],
                ix: 6,
              },
            },
            ao: 0,
            shapes: [
              {
                ty: "gr",
                it: [
                  {
                    d: 1,
                    ty: "el",
                    s: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                          o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                          t: 151,
                          s: [40, 40],
                        },
                        { t: 302, s: [24, 24] },
                      ],
                      ix: 2,
                    },
                    p: { a: 0, k: [0, 0], ix: 3 },
                    nm: "椭圆路径 1",
                    mn: "ADBE Vector Shape - Ellipse",
                    hd: false,
                  },
                  {
                    ty: "fl",
                    c: { a: 0, k: [1, 1, 1, 1], ix: 4 },
                    o: { a: 0, k: 100, ix: 5 },
                    r: 1,
                    bm: 0,
                    nm: "填充 1",
                    mn: "ADBE Vector Graphic - Fill",
                    hd: false,
                  },
                  {
                    ty: "gr",
                    it: [
                      {
                        d: 1,
                        ty: "el",
                        s: {
                          a: 1,
                          k: [
                            {
                              i: { x: [0.833, 0.833], y: [0.833, 0.833] },
                              o: { x: [0.167, 0.167], y: [0.167, 0.167] },
                              t: 151,
                              s: [40, 40],
                            },
                            { t: 302, s: [24, 24] },
                          ],
                          ix: 2,
                        },
                        p: { a: 0, k: [0, 0], ix: 3 },
                        nm: "椭圆路径 1",
                        mn: "ADBE Vector Shape - Ellipse",
                        hd: false,
                      },
                      {
                        ty: "fl",
                        c: { a: 0, k: [1, 1, 1, 1], ix: 4 },
                        o: { a: 0, k: 100, ix: 5 },
                        r: 1,
                        bm: 0,
                        nm: "填充 1",
                        mn: "ADBE Vector Graphic - Fill",
                        hd: false,
                      },
                      {
                        ty: "tr",
                        p: { a: 0, k: [472.605, -2.23], ix: 2 },
                        a: { a: 0, k: [0, 0], ix: 1 },
                        s: { a: 0, k: [100, 100], ix: 3 },
                        r: { a: 0, k: 0, ix: 6 },
                        o: { a: 0, k: 100, ix: 7 },
                        sk: { a: 0, k: 0, ix: 4 },
                        sa: { a: 0, k: 0, ix: 5 },
                        nm: "Transform",
                      },
                    ],
                    nm: "椭圆 1",
                    np: 2,
                    cix: 2,
                    bm: 0,
                    ix: 3,
                    mn: "ADBE Vector Group",
                    hd: false,
                  },
                  {
                    ty: "tr",
                    p: { a: 0, k: [-173.832, -108.902], ix: 2 },
                    a: { a: 0, k: [0, 0], ix: 1 },
                    s: { a: 0, k: [100, 100], ix: 3 },
                    r: { a: 0, k: 0, ix: 6 },
                    o: { a: 0, k: 100, ix: 7 },
                    sk: { a: 0, k: 0, ix: 4 },
                    sa: { a: 0, k: 0, ix: 5 },
                    nm: "Transform",
                  },
                ],
                nm: "椭圆 1",
                np: 3,
                cix: 2,
                bm: 0,
                ix: 1,
                mn: "ADBE Vector Group",
                hd: false,
              },
            ],
            ip: -9,
            op: 1091,
            st: 151,
            bm: 0,
          },
          {
            ddd: 0,
            ind: 7,
            ty: 4,
            nm: "“环一/环”轮廓 3",
            sr: 1,
            ks: {
              o: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833], y: [0.833] },
                    o: { x: [0.167], y: [0.167] },
                    t: 0,
                    s: [0],
                  },
                  { t: 54, s: [100] },
                ],
                ix: 11,
              },
              r: { a: 0, k: 0, ix: 10 },
              p: {
                a: 1,
                k: [
                  {
                    i: { x: 0.833, y: 0.833 },
                    o: { x: 0.167, y: 0.167 },
                    t: 0,
                    s: [400, 400, 0],
                    to: [0, 0, 0],
                    ti: [0, 0, 0],
                  },
                  { t: 151, s: [400, 400, 0] },
                ],
                ix: 2,
              },
              a: { a: 0, k: [253, 253, 0], ix: 1 },
              s: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: 0,
                    s: [75, 75, 100],
                  },
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: 151,
                    s: [100, 100, 100],
                  },
                  { t: 302, s: [124, 124, 100] },
                ],
                ix: 6,
              },
            },
            ao: 0,
            shapes: [
              {
                ty: "gr",
                it: [
                  {
                    ind: 0,
                    ty: "sh",
                    ix: 1,
                    ks: {
                      a: 0,
                      k: {
                        i: [
                          [-131.444, 0],
                          [0, 131.444],
                          [131.444, 0],
                          [0, -131.444],
                        ],
                        o: [
                          [131.444, 0],
                          [0, -131.444],
                          [-131.444, 0],
                          [0, 131.444],
                        ],
                        v: [
                          [0, 238],
                          [238, 0],
                          [0, -238],
                          [-238, 0],
                        ],
                        c: true,
                      },
                      ix: 2,
                    },
                    nm: "Path 1",
                    mn: "ADBE Vector Shape - Group",
                    hd: false,
                  },
                  {
                    ty: "st",
                    c: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833], y: [1] },
                          o: { x: [0.167], y: [0] },
                          t: 151,
                          s: [0.980000019073, 0.990000009537, 0.990000009537, 1],
                        },
                        {
                          t: 302,
                          s: [0.769999980927, 0.670000016689, 0.889999985695, 1],
                        },
                      ],
                      ix: 3,
                    },
                    o: { a: 0, k: 100, ix: 4 },
                    w: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833], y: [0.833] },
                          o: { x: [0.167], y: [0.167] },
                          t: 151,
                          s: [6],
                        },
                        { t: 302, s: [4] },
                      ],
                      ix: 5,
                    },
                    lc: 1,
                    lj: 1,
                    ml: 10,
                    bm: 0,
                    nm: "描边 1",
                    mn: "ADBE Vector Graphic - Stroke",
                    hd: false,
                  },
                  {
                    ty: "tr",
                    p: { a: 0, k: [253, 253], ix: 2 },
                    a: { a: 0, k: [0, 0], ix: 1 },
                    s: { a: 0, k: [100, 100], ix: 3 },
                    r: { a: 0, k: 0, ix: 6 },
                    o: { a: 0, k: 100, ix: 7 },
                    sk: { a: 0, k: 0, ix: 4 },
                    sa: { a: 0, k: 0, ix: 5 },
                    nm: "Transform",
                  },
                ],
                nm: "组 1",
                np: 2,
                cix: 2,
                bm: 0,
                ix: 1,
                mn: "ADBE Vector Group",
                hd: false,
              },
            ],
            ip: -6,
            op: 1091,
            st: 151,
            bm: 0,
          },
          {
            ddd: 0,
            ind: 8,
            ty: 4,
            nm: "“环一/环”轮廓 2",
            sr: 1,
            ks: {
              o: { a: 0, k: 100, ix: 11 },
              r: { a: 0, k: 0, ix: 10 },
              p: { a: 0, k: [400, 400, 0], ix: 2 },
              a: { a: 0, k: [253, 253, 0], ix: 1 },
              s: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: 0,
                    s: [100, 100, 100],
                  },
                  { t: 151, s: [124, 124, 100] },
                ],
                ix: 6,
              },
            },
            ao: 0,
            shapes: [
              {
                ty: "gr",
                it: [
                  {
                    ind: 0,
                    ty: "sh",
                    ix: 1,
                    ks: {
                      a: 0,
                      k: {
                        i: [
                          [-131.444, 0],
                          [0, 131.444],
                          [131.444, 0],
                          [0, -131.444],
                        ],
                        o: [
                          [131.444, 0],
                          [0, -131.444],
                          [-131.444, 0],
                          [0, 131.444],
                        ],
                        v: [
                          [0, 238],
                          [238, 0],
                          [0, -238],
                          [-238, 0],
                        ],
                        c: true,
                      },
                      ix: 2,
                    },
                    nm: "Path 1",
                    mn: "ADBE Vector Shape - Group",
                    hd: false,
                  },
                  {
                    ty: "st",
                    c: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833], y: [1] },
                          o: { x: [0.167], y: [0] },
                          t: 0,
                          s: [0.980000019073, 0.990000009537, 0.990000009537, 1],
                        },
                        {
                          i: { x: [0.833], y: [1] },
                          o: { x: [0.167], y: [0] },
                          t: 149,
                          s: [0.769999980927, 0.680000007153, 0.899999976158, 1],
                        },
                        {
                          t: 151,
                          s: [0.769999980927, 0.670000016689, 0.889999985695, 1],
                        },
                      ],
                      ix: 3,
                    },
                    o: { a: 0, k: 100, ix: 4 },
                    w: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833], y: [0.833] },
                          o: { x: [0.167], y: [0.167] },
                          t: 0,
                          s: [6],
                        },
                        { t: 151, s: [4] },
                      ],
                      ix: 5,
                    },
                    lc: 1,
                    lj: 1,
                    ml: 10,
                    bm: 0,
                    nm: "描边 1",
                    mn: "ADBE Vector Graphic - Stroke",
                    hd: false,
                  },
                  {
                    ty: "tr",
                    p: { a: 0, k: [253, 253], ix: 2 },
                    a: { a: 0, k: [0, 0], ix: 1 },
                    s: { a: 0, k: [100, 100], ix: 3 },
                    r: { a: 0, k: 0, ix: 6 },
                    o: { a: 0, k: 100, ix: 7 },
                    sk: { a: 0, k: 0, ix: 4 },
                    sa: { a: 0, k: 0, ix: 5 },
                    nm: "Transform",
                  },
                ],
                nm: "组 1",
                np: 2,
                cix: 2,
                bm: 0,
                ix: 1,
                mn: "ADBE Vector Group",
                hd: false,
              },
            ],
            ip: 0,
            op: 940,
            st: 0,
            bm: 0,
          },
          {
            ddd: 0,
            ind: 9,
            ty: 4,
            nm: "“环二/环”轮廓 2",
            sr: 1,
            ks: {
              o: { a: 0, k: 100, ix: 11 },
              r: { a: 0, k: 0, ix: 10 },
              p: { a: 0, k: [400, 400, 0], ix: 2 },
              a: { a: 0, k: [298, 298, 0], ix: 1 },
              s: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: 0,
                    s: [100, 100, 100],
                  },
                  { t: 149, s: [118, 118, 100] },
                ],
                ix: 6,
              },
            },
            ao: 0,
            shapes: [
              {
                ty: "gr",
                it: [
                  {
                    ind: 0,
                    ty: "sh",
                    ix: 1,
                    ks: {
                      a: 0,
                      k: {
                        i: [
                          [-163.476, 0],
                          [0, 163.476],
                          [163.476, 0],
                          [0, -163.476],
                        ],
                        o: [
                          [163.476, 0],
                          [0, -163.476],
                          [-163.476, 0],
                          [0, 163.476],
                        ],
                        v: [
                          [0, 296],
                          [296, 0],
                          [0, -296],
                          [-296, 0],
                        ],
                        c: true,
                      },
                      ix: 2,
                    },
                    nm: "Path 1",
                    mn: "ADBE Vector Shape - Group",
                    hd: false,
                  },
                  {
                    ty: "st",
                    c: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833], y: [1] },
                          o: { x: [0.167], y: [0] },
                          t: 0,
                          s: [0.769999980927, 0.670000016689, 0.889999985695, 1],
                        },
                        {
                          t: 151,
                          s: [0.670000016689, 0.52999997139, 0.870000004768, 1],
                        },
                      ],
                      ix: 3,
                    },
                    o: { a: 0, k: 100, ix: 4 },
                    w: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833], y: [0.833] },
                          o: { x: [0.167], y: [0.167] },
                          t: 2,
                          s: [4],
                        },
                        { t: 149, s: [2] },
                      ],
                      ix: 5,
                    },
                    lc: 1,
                    lj: 1,
                    ml: 10,
                    bm: 0,
                    nm: "描边 1",
                    mn: "ADBE Vector Graphic - Stroke",
                    hd: false,
                  },
                  {
                    ty: "tr",
                    p: { a: 0, k: [298, 298], ix: 2 },
                    a: { a: 0, k: [0, 0], ix: 1 },
                    s: { a: 0, k: [100, 100], ix: 3 },
                    r: { a: 0, k: 0, ix: 6 },
                    o: { a: 0, k: 100, ix: 7 },
                    sk: { a: 0, k: 0, ix: 4 },
                    sa: { a: 0, k: 0, ix: 5 },
                    nm: "Transform",
                  },
                ],
                nm: "组 1",
                np: 2,
                cix: 2,
                bm: 0,
                ix: 1,
                mn: "ADBE Vector Group",
                hd: false,
              },
            ],
            ip: 0,
            op: 940,
            st: 0,
            bm: 0,
          },
          {
            ddd: 0,
            ind: 10,
            ty: 4,
            nm: "“环三/环”轮廓 2",
            sr: 1,
            ks: {
              o: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833], y: [0.833] },
                    o: { x: [0.167], y: [0.167] },
                    t: 0,
                    s: [100],
                  },
                  { t: 72, s: [0] },
                ],
                ix: 11,
              },
              r: { a: 0, k: 0, ix: 10 },
              p: { a: 0, k: [400.5, 400.5, 0], ix: 2 },
              a: { a: 0, k: [349.5, 349.5, 0], ix: 1 },
              s: {
                a: 1,
                k: [
                  {
                    i: { x: [0.833, 0.833, 0.833], y: [0.833, 0.833, 1] },
                    o: { x: [0.167, 0.167, 0.167], y: [0.167, 0.167, 0] },
                    t: 0,
                    s: [100, 100, 100],
                  },
                  { t: 151, s: [113, 113, 100] },
                ],
                ix: 6,
              },
            },
            ao: 0,
            shapes: [
              {
                ty: "gr",
                it: [
                  {
                    ind: 0,
                    ty: "sh",
                    ix: 1,
                    ks: {
                      a: 0,
                      k: {
                        i: [
                          [-192.195, 0],
                          [0, 192.195],
                          [192.195, 0],
                          [0, -192.195],
                        ],
                        o: [
                          [192.195, 0],
                          [0, -192.195],
                          [-192.195, 0],
                          [0, 192.195],
                        ],
                        v: [
                          [0, 348],
                          [348, 0],
                          [0, -348],
                          [-348, 0],
                        ],
                        c: true,
                      },
                      ix: 2,
                    },
                    nm: "Path 1",
                    mn: "ADBE Vector Shape - Group",
                    hd: false,
                  },
                  {
                    ty: "st",
                    c: {
                      a: 0,
                      k: [0.768627464771, 0.309803932905, 0.670588254929, 1],
                      ix: 3,
                    },
                    o: { a: 0, k: 100, ix: 4 },
                    w: {
                      a: 1,
                      k: [
                        {
                          i: { x: [0.833], y: [0.833] },
                          o: { x: [0.167], y: [0.167] },
                          t: 2,
                          s: [2],
                        },
                        { t: 151, s: [0] },
                      ],
                      ix: 5,
                    },
                    lc: 1,
                    lj: 1,
                    ml: 10,
                    bm: 0,
                    nm: "描边 1",
                    mn: "ADBE Vector Graphic - Stroke",
                    hd: false,
                  },
                  {
                    ty: "tr",
                    p: { a: 0, k: [349, 349], ix: 2 },
                    a: { a: 0, k: [0, 0], ix: 1 },
                    s: { a: 0, k: [100, 100], ix: 3 },
                    r: { a: 0, k: 0, ix: 6 },
                    o: { a: 0, k: 100, ix: 7 },
                    sk: { a: 0, k: 0, ix: 4 },
                    sa: { a: 0, k: 0, ix: 5 },
                    nm: "Transform",
                  },
                ],
                nm: "组 1",
                np: 2,
                cix: 2,
                bm: 0,
                ix: 1,
                mn: "ADBE Vector Group",
                hd: false,
              },
            ],
            ip: -1,
            op: 942,
            st: 2,
            bm: 0,
          },
        ],
      },
    ],
    layers: [
      {
        ddd: 0,
        ind: 1,
        ty: 0,
        nm: "Lottie_Main_Comp",
        refId: "comp_0",
        sr: 1,
        ks: {
          o: { a: 0, k: 100, ix: 11 },
          r: { a: 0, k: 0, ix: 10 },
          p: { a: 0, k: [400, 400, 0], ix: 2 },
          a: { a: 0, k: [400, 400, 0], ix: 1 },
          s: { a: 0, k: [100, 100, 100], ix: 6 },
        },
        ao: 0,
        w: 800,
        h: 800,
        ip: 0,
        op: 151,
        st: 0,
        bm: 0,
      },
    ],
    markers: [],
  };

  console.log('el', element, animation.v);

  lottie.loadAnimation({
    container: element, // the dom element that will contain the animation
    renderer: 'svg',
    loop: true,
    autoplay: true,
    animationData: animation // the path to the animation json
  });

  lottie.setSpeed(2.5);

  </script>`;

  return (
    <iframe
      style={{
        width: width,
        height: height,
      }}
      srcDoc={source}
    />
  );
};

const NearReward = ({ amount }) => {
  const amountInNear = Big(Big(amount).div(1e24).toFixed(2))
    .toNumber()
    .toString();

  return (
    <RewardBoundary rarity={props.rarity}>
      <NearRewardAmount>{amountInNear}</NearRewardAmount>
      <NearRewardIcon width="80%" />
    </RewardBoundary>
  );
};

const getMediaUrlForToken = (contract_id, token_id) => {
  const metadata = Near.view(contract_id, 'nft_metadata');
  const token = Near.view(contract_id, 'nft_token', { token_id });

  if (!metadata || !token) return null;

  if (!token.metadata?.media) return undefined;

  if (!metadata.base_uri) return token.metadata?.media;

  const url = metadata.base_uri + '/' + token.metadata.media;

  return url;
};

const NonFungibleTokenReward = ({ contract_id, token_id }) => {
  const url = getMediaUrlForToken(contract_id, token_id);

  console.log(`Loading NFT image from ${url}`);

  return (
    <RewardBoundary rarity={props.rarity}>
      <NonFungibleTokenRewardImage src={url} />
    </RewardBoundary>
  );
};

const NothingReward = () => {
  return (
    <>
      <PrimaryText>
        Fortune has departed this time, but do not dismay!
      </PrimaryText>
      <PrimaryText>Join us next time and prepare for cosmic luck!</PrimaryText>
    </>
  );
};

const ContentWrapperComponent = ({ rarity, children }) => {
  const backgrounds = {
    rare: {
      portrait:
        'https://ipfs.near.social/ipfs/bafkreigdlbicksoqijiekfytogvz4cmykpbx2fdmpz5ghiqki2ou4dig2u',
      landscape:
        'https://ipfs.near.social/ipfs/bafkreibyfjqfivipdlavmjet2jdfvywkto2vi7ooi2eli6e4u3iprbwtoi',
    },
    epic: {
      portrait:
        'https://ipfs.near.social/ipfs/bafkreie32u3ab2ml5sqyu6oipo63h4mnbkytplqgcigqy4o5zxvznw4lwi',
      landscape:
        'https://ipfs.near.social/ipfs/bafkreihbthqvggathcry43suj6capp3es6bvhao4e7r73jmqipxvhedske',
    },
    legendary: {
      portrait:
        'https://ipfs.near.social/ipfs/bafkreidyj6fpphpuzve22lkyukqxicqr7sph5hjac4efakyzlzacmbfgbe',
      landscape:
        'https://ipfs.near.social/ipfs/bafkreicxafpy4qeqqh6ww6qkblvy3wc35jcddznjovtk2wbgmv6qamkyna',
    },
  };

  const background = backgrounds[rarity];

  return (
    <ContentWrapper
      portrait={background?.portrait}
      landscape={background?.landscape}
    >
      {children}
    </ContentWrapper>
  );
};

return (
  <Wrapper>
    <ContentWrapperComponent rarity={props.rarity}>
      <div
        style={{
          flexBasis: '55%',
          display: 'flex',
          flexDirection: 'column',
          justifyContent: 'center',
          alignItems: 'center',
        }}
      >
        {!props.reward && <LoadingReward width={196} height={196} />}
        {props.reward?.kind === 'near' && (
          <NearReward amount={props.reward.amount} />
        )}
        {props.reward?.kind === 'non_fungible_token' && (
          <NonFungibleTokenReward
            contract_id={props.reward.contract_id}
            token_id={props.reward.token_id}
          />
        )}
        {props.reward?.kind === 'nothing' && <NothingReward />}
      </div>
      <WhiteButton onClick={onClick}>Back</WhiteButton>
    </ContentWrapperComponent>
  </Wrapper>
);
