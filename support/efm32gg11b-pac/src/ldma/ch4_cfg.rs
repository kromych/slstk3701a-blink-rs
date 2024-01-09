#[doc = "Register `CH4_CFG` reader"]
pub type R = crate::R<CH4_CFG_SPEC>;
#[doc = "Register `CH4_CFG` writer"]
pub type W = crate::W<CH4_CFG_SPEC>;
#[doc = "Field `ARBSLOTS` reader - Arbitration Slot Number Select"]
pub type ARBSLOTS_R = crate::FieldReader<ARBSLOTS_A>;
#[doc = "Arbitration Slot Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBSLOTS_A {
    #[doc = "0: One arbitration slot selected"]
    ONE = 0,
    #[doc = "1: Two arbitration slots selected"]
    TWO = 1,
    #[doc = "2: Four arbitration slots selected"]
    FOUR = 2,
    #[doc = "3: Eight arbitration slots selected"]
    EIGHT = 3,
}
impl From<ARBSLOTS_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBSLOTS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARBSLOTS_A {
    type Ux = u8;
}
impl ARBSLOTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARBSLOTS_A {
        match self.bits {
            0 => ARBSLOTS_A::ONE,
            1 => ARBSLOTS_A::TWO,
            2 => ARBSLOTS_A::FOUR,
            3 => ARBSLOTS_A::EIGHT,
            _ => unreachable!(),
        }
    }
    #[doc = "One arbitration slot selected"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ARBSLOTS_A::ONE
    }
    #[doc = "Two arbitration slots selected"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == ARBSLOTS_A::TWO
    }
    #[doc = "Four arbitration slots selected"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == ARBSLOTS_A::FOUR
    }
    #[doc = "Eight arbitration slots selected"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == ARBSLOTS_A::EIGHT
    }
}
#[doc = "Field `ARBSLOTS` writer - Arbitration Slot Number Select"]
pub type ARBSLOTS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ARBSLOTS_A>;
impl<'a, REG> ARBSLOTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One arbitration slot selected"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSLOTS_A::ONE)
    }
    #[doc = "Two arbitration slots selected"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSLOTS_A::TWO)
    }
    #[doc = "Four arbitration slots selected"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSLOTS_A::FOUR)
    }
    #[doc = "Eight arbitration slots selected"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSLOTS_A::EIGHT)
    }
}
#[doc = "Field `SRCINCSIGN` reader - Source Address Increment Sign"]
pub type SRCINCSIGN_R = crate::BitReader;
#[doc = "Field `SRCINCSIGN` writer - Source Address Increment Sign"]
pub type SRCINCSIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSTINCSIGN` reader - Destination Address Increment Sign"]
pub type DSTINCSIGN_R = crate::BitReader;
#[doc = "Field `DSTINCSIGN` writer - Destination Address Increment Sign"]
pub type DSTINCSIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline(always)]
    pub fn arbslots(&self) -> ARBSLOTS_R {
        ARBSLOTS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline(always)]
    pub fn srcincsign(&self) -> SRCINCSIGN_R {
        SRCINCSIGN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline(always)]
    pub fn dstincsign(&self) -> DSTINCSIGN_R {
        DSTINCSIGN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline(always)]
    #[must_use]
    pub fn arbslots(&mut self) -> ARBSLOTS_W<CH4_CFG_SPEC> {
        ARBSLOTS_W::new(self, 16)
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline(always)]
    #[must_use]
    pub fn srcincsign(&mut self) -> SRCINCSIGN_W<CH4_CFG_SPEC> {
        SRCINCSIGN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline(always)]
    #[must_use]
    pub fn dstincsign(&mut self) -> DSTINCSIGN_W<CH4_CFG_SPEC> {
        DSTINCSIGN_W::new(self, 21)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4_CFG_SPEC;
impl crate::RegisterSpec for CH4_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_cfg::R`](R) reader structure"]
impl crate::Readable for CH4_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch4_cfg::W`](W) writer structure"]
impl crate::Writable for CH4_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH4_CFG to value 0"]
impl crate::Resettable for CH4_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
