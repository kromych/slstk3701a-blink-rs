#[doc = "Register `MIR1_ARB` reader"]
pub type R = crate::R<MIR1_ARB_SPEC>;
#[doc = "Register `MIR1_ARB` writer"]
pub type W = crate::W<MIR1_ARB_SPEC>;
#[doc = "Field `ID` reader - Message Identifier"]
pub type ID_R = crate::FieldReader<u32>;
#[doc = "Field `ID` writer - Message Identifier"]
pub type ID_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `DIR` reader - Message Direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Message Direction"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTD` reader - Extended Identifier"]
pub type XTD_R = crate::BitReader;
#[doc = "Field `XTD` writer - Extended Identifier"]
pub type XTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSGVAL` reader - Message Valid"]
pub type MSGVAL_R = crate::BitReader;
#[doc = "Field `MSGVAL` writer - Message Valid"]
pub type MSGVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:28 - Message Identifier"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Message Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&self) -> XTD_R {
        XTD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Message Valid"]
    #[inline(always)]
    pub fn msgval(&self) -> MSGVAL_R {
        MSGVAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Message Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<MIR1_ARB_SPEC> {
        ID_W::new(self, 0)
    }
    #[doc = "Bit 29 - Message Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<MIR1_ARB_SPEC> {
        DIR_W::new(self, 29)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn xtd(&mut self) -> XTD_W<MIR1_ARB_SPEC> {
        XTD_W::new(self, 30)
    }
    #[doc = "Bit 31 - Message Valid"]
    #[inline(always)]
    #[must_use]
    pub fn msgval(&mut self) -> MSGVAL_W<MIR1_ARB_SPEC> {
        MSGVAL_W::new(self, 31)
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
#[doc = "Interface Arbitration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_arb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_arb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIR1_ARB_SPEC;
impl crate::RegisterSpec for MIR1_ARB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir1_arb::R`](R) reader structure"]
impl crate::Readable for MIR1_ARB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mir1_arb::W`](W) writer structure"]
impl crate::Writable for MIR1_ARB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIR1_ARB to value 0"]
impl crate::Resettable for MIR1_ARB_SPEC {
    const RESET_VALUE: u32 = 0;
}
