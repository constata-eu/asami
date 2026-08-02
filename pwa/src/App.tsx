import { useEffect, useMemo, useState } from "react";
import logo from "./assets/logo.svg";
import collective from "./assets/rootstock_collective.svg";

type Member = { id: string; name: string; address: string; score: string };
type Asset = { address: string; minimumAmount: string };
type Config = {
  chainId: number;
  contractAddress: string;
  explorerUrl: string;
  contractUrl: string;
  senderRegistrationUrl: string;
  xEnglish: string;
};

const zero = "0x0000000000000000000000000000000000000000";
const handlePattern = /^@[A-Za-z0-9_]{1,15}$/;
const addressPattern = /^0x[0-9a-fA-F]{40}$/;

const copy = {
  en: {
    eyebrow: "INVITATION ONLY",
    title: "A club to thank and reward Rootstock advocates",
    intro:
      "Talk about Rootstock, Bitcoin, and any associated project, we'll notice you, and invite you.",
    gift: "Send a thank-you",
    giftText:
      "Anyone can airdrop funds to Rootstockers. Send any asset to the club contract and Asami will share it among current members.",
    contract: "Club contract",
    copy: "Copy",
    copied: "Copied",
    registerText:
      "Sending a gift? Register your name directly on the contract.",
    contactUsText:
      "Or mention Asami on X with your address and we can register it for you.",
    credits: "With gratitude to",
    empty: "The member list is being synchronized.",
    error: "The club data is temporarily unavailable.",
  },
  es: {
    eyebrow: "SOLO POR INVITACIÓN",
    title:
      "Un club para agradecer y recompensar a los divulgadores de Rootstock",
    intro:
      "Habla sobre Rootstock, Bitcoin, y cualquier proyecto asociado, prestaremos atención, y te invitaremos.",
    gift: "Envía un agradecimiento",
    giftText:
      "Cualquiera puede enviar fondos a los Rootstockers. Envía cualquier activo al contrato del club y Asami lo compartirá entre los miembros actuales.",
    contract: "Contrato del club",
    copy: "Copiar",
    copied: "Copiado",
    registerText:
      "¿Envías un regalo? Registra tu nombre directamente en el contrato",
    contactUsText:
      "O menciona a Asami en X con tu dirección para que podamos registrarlo.",
    credits: "Con gratitud a",
    empty: "La lista de miembros se está sincronizando.",
    error: "Los datos del club no están disponibles temporalmente.",
  },
} as const;

function browserLanguage(): keyof typeof copy {
  return navigator.language.toLowerCase().startsWith("es") ? "es" : "en";
}

export function App() {
  const [members, setMembers] = useState<Member[]>([]);
  const [assets, setAssets] = useState<Asset[]>([]);
  const [config, setConfig] = useState<Config | null>(null);
  const [failed, setFailed] = useState(false);
  const [copied, setCopied] = useState(false);
  const language = useMemo(browserLanguage, []);
  const text = copy[language];

  useEffect(() => {
    Promise.all([
      fetch("/members").then((response) => checked<Member[]>(response)),
      fetch("/assets").then((response) => checked<Asset[]>(response)),
      fetch("/config").then((response) => checked<Config>(response)),
    ])
      .then(([nextMembers, nextAssets, nextConfig]) => {
        setMembers(nextMembers);
        setAssets(nextAssets);
        setConfig(nextConfig);
      })
      .catch(() => setFailed(true));
  }, []);

  async function copyAddress() {
    if (!config) return;
    await navigator.clipboard.writeText(config.contractAddress);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1600);
  }

  return (
    <main>
      <header className="hero">
        <img className="brand-logo" src={logo} alt="Asami" />
        <p className="eyebrow">{text.eyebrow}</p>
        <h1>{text.title}</h1>
        <p className="intro">{text.intro}</p>
      </header>

      {failed ? <p className="notice error">{text.error}</p> : null}

      <section className="members-section" aria-label="Members">
        {members.length === 0 && !failed ? (
          <p className="notice">{text.empty}</p>
        ) : null}
        <div className="member-grid">
          {members.map((member) => (
            <MemberCard
              key={member.id}
              member={member}
              explorer={config?.explorerUrl}
            />
          ))}
        </div>
      </section>

      <section className="gift-panel">
        <div>
          <p className="eyebrow">{text.gift}</p>
          <h2>{text.contract}</h2>
          <p>{text.giftText}</p>
        </div>
        {config ? (
          <div className="contract-details">
            <a
              className="address"
              href={config.contractUrl}
              target="_blank"
              rel="noreferrer"
            >
              {config.contractAddress}
            </a>
            <button onClick={copyAddress}>
              {copied ? text.copied : text.copy}
            </button>
          </div>
        ) : null}
        {config ? (
          <>
            <p className="registration">
              <a
                href={config.senderRegistrationUrl}
                target="_blank"
                rel="noreferrer"
              >
                {text.registerText}
              </a>{" "}
            </p>
            <p className="registration">
              {text.contactUsText}{" "}
              <a
                href="https://x.com/asami_club"
                target="_blank"
                rel="noreferrer"
              >
                {config.xEnglish}
              </a>
            </p>
          </>
        ) : null}
      </section>

      <footer>
        <span>{text.credits}</span>
        <a
          className="sponsor sponsor-moc"
          href="https://moneyonchain.com/"
          target="_blank"
          rel="noreferrer"
        >
          Money on Chain
        </a>
        <a
          className="sponsor"
          href="https://rootstockcollective.xyz/"
          target="_blank"
          rel="noreferrer"
        >
          <img src={collective} alt="Rootstock Collective" />
        </a>
      </footer>
    </main>
  );
}

function MemberCard({
  member,
  explorer,
}: {
  member: Member;
  explorer?: string;
}) {
  const name = member.name.trim() || member.address;
  const isHandle = handlePattern.test(name);
  const isAddress = addressPattern.test(name);
  const href = isHandle
    ? "https://x.com/" + name.slice(1)
    : isAddress && explorer
      ? explorer + "/address/" + name
      : null;

  return (
    <article className="member-card">
      <div
        className="avatar"
        dangerouslySetInnerHTML={{ __html: identicon(member.address) }}
      />
      <h3 className="member-name">
        {href ? (
          <a href={href} target="_blank" rel="noreferrer">
            {name}
          </a>
        ) : (
          name
        )}
      </h3>
    </article>
  );
}

function identicon(address: string): string {
  let hash = 2166136261;
  for (const character of address.toLowerCase()) {
    hash ^= character.charCodeAt(0);
    hash = Math.imul(hash, 16777619);
  }
  const hue = Math.abs(hash) % 360;
  const cells: string[] = [];
  let state = hash >>> 0;
  for (let y = 0; y < 5; y += 1) {
    for (let x = 0; x < 3; x += 1) {
      state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
      if ((state & 1) === 1) {
        cells.push('<rect x="' + x + '" y="' + y + '" width="1" height="1"/>');
        if (x !== 2)
          cells.push(
            '<rect x="' + (4 - x) + '" y="' + y + '" width="1" height="1"/>',
          );
      }
    }
  }
  return (
    '<svg viewBox="0 0 5 5" role="img" aria-label="Address image" style="color:hsl(' +
    hue +
    ' 66% 48%)"><rect width="5" height="5" fill="hsl(' +
    hue +
    ' 70% 94%)"/><g fill="currentColor">' +
    cells.join("") +
    "</g></svg>"
  );
}

function shortAddress(address: string): string {
  return address.slice(0, 8) + "…" + address.slice(-6);
}

async function checked<T>(response: Response): Promise<T> {
  if (!response.ok) throw new Error("request failed");
  return response.json() as Promise<T>;
}
