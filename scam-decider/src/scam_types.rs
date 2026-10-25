use std::fmt;

use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoStaticStr, VariantArray};

// the kinds of financial scams we can label a message as
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
    VariantArray,
    EnumString,
    IntoStaticStr,
    Serialize,
    Deserialize,
)]
pub enum ScamType {
    // not a scam at all, a clean message
    #[default]
    NotAScam,
    // promise of a large payout in exchange for an upfront payment
    AdvanceFee,
    // the "Nigerian prince" letter, a stranger needs help moving a fortune
    InheritanceScam,
    // old investors are paid with money from new investors
    PonziScheme,
    // recruits pay to join and earn by recruiting more people
    PyramidScheme,
    // "guaranteed" high returns on a fake investment or trading platform
    InvestmentFraud,
    // long-con romance that ends in an "investment" or emergency payment
    RomanceScam,
    // buyer overpays with a bad check or transfer and asks for the difference back
    OverpaymentFraud,
    // inflated hype pushes a cheap asset so the promoter can dump it
    PumpAndDump,
    // crypto project drains liquidity and disappears
    RugPull,
    // victim is walked through fake deposits until their savings are gone
    PigButchering,
    // lottery, sweepstake or prize that requires a fee or details to claim
    PrizeScam,
    // fake refund or asset recovery offer targeting a previous victim
    RecoveryScam,
    // debt relief or credit repair that charges fees and delivers nothing
    DebtRelief,
    // loan offer that asks for insurance, tax or processing fees first
    LoanFeeScam,
    // job offer that demands payment for training, equipment or onboarding
    FakeJobOffer,
    // fake invoice or supplier email tricking a business into paying
    InvoiceFraud,
    // scammer poses as a relative, official or bank and asks for money urgently
    Impersonation,
    // free trial or cheap subscription that hides recurring charges
    SubscriptionTrap,
    // pays existing members for recruiting instead of selling a real product
    MultilevelMarketing,
    // scammer offers to launder or move money through the victim's account
    MoneyMule,
    // "send gift cards" used as untraceable payment in another scam
    GiftCardPayment,
    // fake charity or disaster appeal collecting donations
    FakeCharity,
    // unsolicited crypto airdrop, giveaway or doubling scheme
    CryptoGiveaway,
    // cold call offering to "fix" a computer, account or security problem
    TechSupport,
    // a scam that does not match any of the categories above
    Other,
}

impl ScamType {
    // every variant, handy for prompts, labels and UI lists
    pub const ALL: &'static [ScamType] = &[
        ScamType::NotAScam,
        ScamType::AdvanceFee,
        ScamType::InheritanceScam,
        ScamType::PonziScheme,
        ScamType::PyramidScheme,
        ScamType::InvestmentFraud,
        ScamType::RomanceScam,
        ScamType::OverpaymentFraud,
        ScamType::PumpAndDump,
        ScamType::RugPull,
        ScamType::PigButchering,
        ScamType::PrizeScam,
        ScamType::RecoveryScam,
        ScamType::DebtRelief,
        ScamType::LoanFeeScam,
        ScamType::FakeJobOffer,
        ScamType::InvoiceFraud,
        ScamType::Impersonation,
        ScamType::SubscriptionTrap,
        ScamType::MultilevelMarketing,
        ScamType::MoneyMule,
        ScamType::GiftCardPayment,
        ScamType::FakeCharity,
        ScamType::CryptoGiveaway,
        ScamType::TechSupport,
        ScamType::Other,
    ];

    // one line about what the scam is
    pub fn description(&self) -> &'static str {
        match self {
            ScamType::NotAScam => "No scam detected, the message looks legitimate.",
            ScamType::AdvanceFee => {
                "Asks for an upfront payment before a promised payout is released."
            }
            ScamType::InheritanceScam => {
                "A stranger claims to hold a fortune they need help moving out of the country."
            }
            ScamType::PonziScheme => {
                "Pays earlier investors with money from newer investors instead of real profit."
            }
            ScamType::PyramidScheme => {
                "Charges a joining fee and rewards members mainly for recruiting more members."
            }
            ScamType::InvestmentFraud => {
                "Advertises guaranteed or outsized returns through a fake broker or trading platform."
            }
            ScamType::RomanceScam => {
                "Builds a fake relationship online, then asks for money or a crypto investment."
            }
            ScamType::OverpaymentFraud => {
                "Overpays with a fraudulent check or transfer and asks for the excess back."
            }
            ScamType::PumpAndDump => {
                "Hyped up a cheap asset so insiders can sell at the inflated price."
            }
            ScamType::RugPull => {
                "A crypto project takes investors' funds and disappears, leaving the token worthless."
            }
            ScamType::PigButchering => {
                "Grooms the victim and walks them through fake deposits until their savings are gone."
            }
            ScamType::PrizeScam => {
                "Claims a lottery, sweepstake or prize win, then asks for fees or personal details."
            }
            ScamType::RecoveryScam => {
                "Offers to recover money lost in an earlier scam, for a fee paid in advance."
            }
            ScamType::DebtRelief => {
                "Promises to erase debt or repair credit for fees, and delivers nothing."
            }
            ScamType::LoanFeeScam => {
                "Offers a loan but demands insurance, tax or processing fees before it is paid out."
            }
            ScamType::FakeJobOffer => {
                "A job offer that requires paying for training, equipment or onboarding."
            }
            ScamType::InvoiceFraud => {
                "Sends a fake invoice or supplier email to redirect a business payment."
            }
            ScamType::Impersonation => {
                "Impersonates a relative, bank or official to demand an urgent payment."
            }
            ScamType::SubscriptionTrap => {
                "A free trial or cheap plan that hides steep recurring charges."
            }
            ScamType::MultilevelMarketing => {
                "Sells a recruiting scheme disguised as a business opportunity or product."
            }
            ScamType::MoneyMule => {
                "Asks the victim to receive or move someone else's money through their account."
            }
            ScamType::GiftCardPayment => {
                "Requests gift cards as payment to keep the transfer untraceable."
            }
            ScamType::FakeCharity => {
                "Collects donations for a charity or disaster that does not exist."
            }
            ScamType::CryptoGiveaway => {
                "Offers to double crypto or hands out airdrops in exchange for wallet access."
            }
            ScamType::TechSupport => {
                "Cold call or pop-up claiming a device or account is compromised and needs paid help."
            }
            ScamType::Other => "A scam that does not fit any of the other categories.",
        }
    }

    // red flags that usually show up for this scam, empty for NotAScam
    pub fn telltales(&self) -> &'static [&'static str] {
        match self {
            ScamType::NotAScam => &["It is not.", "It isn't."],
            ScamType::AdvanceFee => &[
                "A large payout is promised but only after a smaller payment.",
                "The fee is labelled as tax, customs, insurance or a release charge.",
                "Payment is pushed to a wire, crypto wallet or gift card.",
                "There is a deadline and the offer expires if you hesitate.",
            ],
            ScamType::InheritanceScam => &[
                "A stranger claims to be a prince, lawyer or bank official with a fortune.",
                "An unclaimed inheritance or dormant account needs a foreign partner.",
                "Fees for legal papers, taxes or bribes are due before the payout.",
                "The story is vague and changes, and secrecy is requested.",
            ],
            ScamType::PonziScheme => &[
                "Returns are steady and smooth, never dropping even when markets fall.",
                "Profits are described as coming from other members or referrals.",
                "Withdrawals get delayed or blocked while deposits are still welcome.",
                "Statements come from a private portal, never a licensed broker.",
            ],
            ScamType::PyramidScheme => &[
                "Income depends mostly on how many people you recruit.",
                "You must buy a starter kit, stock or membership to join.",
                "Real product sales are vague or secondary to recruitment.",
                "Upline earnings are shown off as proof it works.",
            ],
            ScamType::InvestmentFraud => &[
                "Guaranteed returns or a fixed daily percentage.",
                "Returns shown on a dashboard you cannot independently verify.",
                "Withdrawal unlocks a new fee, tax or minimum balance.",
                "The broker is unlicensed and contact happens on chat apps only.",
            ],
            ScamType::RomanceScam => &[
                "The relationship moves fast and stays away from video calls.",
                "A sudden emergency, hospital bill or visa fee needs money now.",
                "Counterpart pushes an investment or crypto platform they control.",
                "Requests shift to gift cards, crypto or payment to a third party.",
            ],
            ScamType::OverpaymentFraud => &[
                "Payment arrives for more than the agreed price.",
                "You are asked to refund the difference to a different account.",
                "The check or transfer is later reversed after you send the refund.",
                "Buyer is pushy and offers to overpay to secure the item.",
            ],
            ScamType::PumpAndDump => &[
                "Urgent buy signal on a tiny, thinly traded asset.",
                "Claims of an imminent announcement or listing with no source.",
                "Coordinated hype from many new accounts at once.",
                "The promoter holds a position and tells others to buy now.",
            ],
            ScamType::RugPull => &[
                "Anonymous team with no audit and no verifiable contract.",
                "Liquidity is unlocked or quietly pulled soon after launch.",
                "Holders are told not to sell while insiders transfer out.",
                "Presale money is collected with no vesting or escrow.",
            ],
            ScamType::PigButchering => &[
                "A friendly wrong-number or social media message starts it.",
                "Trust is built for weeks before any money is mentioned.",
                "A small deposit is allowed to withdraw to prove the platform works.",
                "Every larger withdrawal gets taxed, frozen or penalized.",
            ],
            ScamType::PrizeScam => &[
                "You won a contest you never entered.",
                "A fee, tax or shipping cost is required to release the prize.",
                "Bank details or an ID copy are requested before delivery.",
                "The sender uses a free mail domain instead of an official one.",
            ],
            ScamType::RecoveryScam => &[
                "They contact you first about money you already lost.",
                "A recovery fee, retainer or court cost is due upfront.",
                "No license number, regulator or case reference that checks out.",
                "They promise to trace crypto or hack the scammer back.",
            ],
            ScamType::DebtRelief => &[
                "Debt is promised to be wiped or credit repaired for a fee.",
                "Payment is demanded before any service is performed.",
                "You are told to stop paying creditors and stop contacting them.",
                "No written agreement, license or clear timeline is provided.",
            ],
            ScamType::LoanFeeScam => &[
                "Approval is offered before any real credit check.",
                "Fees for insurance, tax or processing are due first.",
                "The lender has no license and only a chat app contact.",
                "Terms are unusually generous, then the money never arrives.",
            ],
            ScamType::FakeJobOffer => &[
                "The job is offered without a real interview.",
                "You must pay for training, software, equipment or a background check.",
                "Pay is far above market for simple remote tasks.",
                "Onboarding asks for bank details or to move company money.",
            ],
            ScamType::InvoiceFraud => &[
                "Bank details suddenly changed on a known supplier invoice.",
                "The email domain is one letter off the real one.",
                "Payment is marked urgent and confidentiality is requested.",
                "Reply-to address differs from the sender address.",
            ],
            ScamType::Impersonation => &[
                "Claims to be a relative, official, police or bank staff.",
                "Urgency and secrecy, with a warning not to tell anyone.",
                "Requests payment by gift card, crypto or wire transfer.",
                "Contact number or address does not match the real institution.",
            ],
            ScamType::SubscriptionTrap => &[
                "A free trial needs card details for a tiny verification charge.",
                "The real recurring price is hidden in tiny print.",
                "Cancelling requires a phone call during limited hours.",
                "Charges appear under an unrecognized company name.",
            ],
            ScamType::MultilevelMarketing => &[
                "Income claims come from recruiting, not from product sales.",
                "Starter packs, tiers and ranks cost money to advance.",
                "Testimonials focus on lifestyle and cars, not customers.",
                "Pressure to buy inventory you must then resell yourself.",
            ],
            ScamType::MoneyMule => &[
                "You are asked to receive money and forward it on.",
                "Use of your own bank account or crypto wallet is requested.",
                "A commission is offered for the transfer, with no real explanation.",
                "The sender says the funds are for a relative or business you cannot verify.",
            ],
            ScamType::GiftCardPayment => &[
                "Payment is requested in gift card codes.",
                "You are told to read the numbers off the back of the card.",
                "The request comes with urgency or a threat of legal action.",
                "Any other payment method is refused.",
            ],
            ScamType::FakeCharity => &[
                "Donations are requested by gift card, crypto or wire only.",
                "The charity name is close to, but not exactly, a real one.",
                "No registration number or verifiable address is given.",
                "Images and stories are reused from old news reports.",
            ],
            ScamType::CryptoGiveaway => &[
                "Send crypto to receive double back.",
                "A giveaway is tied to a fake executive or live stream.",
                "Wallet seed phrase or private key is requested.",
                "The site imitates a known exchange or brand.",
            ],
            ScamType::TechSupport => &[
                "An unsolicited call or pop-up claims your device is infected.",
                "Remote access software is requested to 'fix' it.",
                "Payment for support is asked in gift cards or crypto.",
                "The caller knows only generic details and pressures you to act now.",
            ],
            ScamType::Other => &[
                "The message asks for money or personal details but fits no known scam pattern.",
                "Urgency, a threat or a too-good-to-be-true promise is used to get a payment.",
                "The sender or company cannot be verified, and only untraceable payment methods are offered.",
                "The story does not hold up and the details change when questioned.",
            ],
        }
    }

    // true for anything except NotAScam
    pub fn is_scam(&self) -> bool {
        !matches!(self, ScamType::NotAScam)
    }
}

impl fmt::Display for ScamType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ScamType::NotAScam => "Not a scam",
            ScamType::AdvanceFee => "Advance fee",
            ScamType::InheritanceScam => "Inheritance scam (Nigerian prince)",
            ScamType::PonziScheme => "Ponzi scheme",
            ScamType::PyramidScheme => "Pyramid scheme",
            ScamType::InvestmentFraud => "Investment fraud",
            ScamType::RomanceScam => "Romance scam",
            ScamType::OverpaymentFraud => "Overpayment fraud",
            ScamType::PumpAndDump => "Pump and dump",
            ScamType::RugPull => "Rug pull",
            ScamType::PigButchering => "Pig butchering",
            ScamType::PrizeScam => "Prize scam",
            ScamType::RecoveryScam => "Recovery scam",
            ScamType::DebtRelief => "Debt relief scam",
            ScamType::LoanFeeScam => "Loan fee scam",
            ScamType::FakeJobOffer => "Fake job offer",
            ScamType::InvoiceFraud => "Invoice fraud",
            ScamType::Impersonation => "Impersonation",
            ScamType::SubscriptionTrap => "Subscription trap",
            ScamType::MultilevelMarketing => "Multilevel marketing",
            ScamType::MoneyMule => "Money mule",
            ScamType::GiftCardPayment => "Gift card payment",
            ScamType::FakeCharity => "Fake charity",
            ScamType::CryptoGiveaway => "Crypto giveaway",
            ScamType::TechSupport => "Tech support scam",
            ScamType::Other => "Other",
        };

        f.write_str(name)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use strum::VariantArray;

    use super::*;

    #[test]
    fn all_lists_every_variant() {
        assert_eq!(ScamType::ALL, ScamType::VARIANTS);
    }

    #[test]
    fn other_labels_unclassified_scams() {
        let other = ScamType::Other;

        assert!(other.is_scam());
        assert!(!other.description().is_empty());
        assert!(!other.telltales().is_empty());
        assert_eq!(ScamType::from_str("Other").unwrap(), other);
        assert_eq!(other.to_string(), "Other");
    }
}
