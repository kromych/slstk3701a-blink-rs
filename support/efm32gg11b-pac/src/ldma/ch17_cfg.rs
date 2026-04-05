#[doc = "Register `CH17_CFG` reader"]
pub type R = crate::R<Ch17CfgSpec>;
#[doc = "Register `CH17_CFG` writer"]
pub type W = crate::W<Ch17CfgSpec>;
#[doc = "Arbitration Slot Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Arbslots {
    #[doc = "0: One arbitration slot selected"]
    One = 0,
    #[doc = "1: Two arbitration slots selected"]
    Two = 1,
    #[doc = "2: Four arbitration slots selected"]
    Four = 2,
    #[doc = "3: Eight arbitration slots selected"]
    Eight = 3,
}
impl From<Arbslots> for u8 {
    #[inline(always)]
    fn from(variant: Arbslots) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Arbslots {
    type Ux = u8;
}
impl crate::IsEnum for Arbslots {}
#[doc = "Field `ARBSLOTS` reader - Arbitration Slot Number Select"]
pub type ArbslotsR = crate::FieldReader<Arbslots>;
impl ArbslotsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arbslots {
        match self.bits {
            0 => Arbslots::One,
            1 => Arbslots::Two,
            2 => Arbslots::Four,
            3 => Arbslots::Eight,
            _ => unreachable!(),
        }
    }
    #[doc = "One arbitration slot selected"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Arbslots::One
    }
    #[doc = "Two arbitration slots selected"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Arbslots::Two
    }
    #[doc = "Four arbitration slots selected"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Arbslots::Four
    }
    #[doc = "Eight arbitration slots selected"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == Arbslots::Eight
    }
}
#[doc = "Field `ARBSLOTS` writer - Arbitration Slot Number Select"]
pub type ArbslotsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Arbslots, crate::Safe>;
impl<'a, REG> ArbslotsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One arbitration slot selected"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Arbslots::One)
    }
    #[doc = "Two arbitration slots selected"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Arbslots::Two)
    }
    #[doc = "Four arbitration slots selected"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Arbslots::Four)
    }
    #[doc = "Eight arbitration slots selected"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(Arbslots::Eight)
    }
}
#[doc = "Field `SRCINCSIGN` reader - Source Address Increment Sign"]
pub type SrcincsignR = crate::BitReader;
#[doc = "Field `SRCINCSIGN` writer - Source Address Increment Sign"]
pub type SrcincsignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSTINCSIGN` reader - Destination Address Increment Sign"]
pub type DstincsignR = crate::BitReader;
#[doc = "Field `DSTINCSIGN` writer - Destination Address Increment Sign"]
pub type DstincsignW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline(always)]
    pub fn arbslots(&self) -> ArbslotsR {
        ArbslotsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline(always)]
    pub fn srcincsign(&self) -> SrcincsignR {
        SrcincsignR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline(always)]
    pub fn dstincsign(&self) -> DstincsignR {
        DstincsignR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline(always)]
    pub fn arbslots(&mut self) -> ArbslotsW<'_, Ch17CfgSpec> {
        ArbslotsW::new(self, 16)
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline(always)]
    pub fn srcincsign(&mut self) -> SrcincsignW<'_, Ch17CfgSpec> {
        SrcincsignW::new(self, 20)
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline(always)]
    pub fn dstincsign(&mut self) -> DstincsignW<'_, Ch17CfgSpec> {
        DstincsignW::new(self, 21)
    }
}
#[doc = "Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch17_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch17_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch17CfgSpec;
impl crate::RegisterSpec for Ch17CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch17_cfg::R`](R) reader structure"]
impl crate::Readable for Ch17CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ch17_cfg::W`](W) writer structure"]
impl crate::Writable for Ch17CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH17_CFG to value 0"]
impl crate::Resettable for Ch17CfgSpec {}
