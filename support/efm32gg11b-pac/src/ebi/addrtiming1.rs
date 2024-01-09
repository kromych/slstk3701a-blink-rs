#[doc = "Register `ADDRTIMING1` reader"]
pub type R = crate::R<ADDRTIMING1_SPEC>;
#[doc = "Register `ADDRTIMING1` writer"]
pub type W = crate::W<ADDRTIMING1_SPEC>;
#[doc = "Field `ADDRSETUP` reader - Address Setup Time"]
pub type ADDRSETUP_R = crate::FieldReader;
#[doc = "Field `ADDRSETUP` writer - Address Setup Time"]
pub type ADDRSETUP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADDRHOLD` reader - Address Hold Time"]
pub type ADDRHOLD_R = crate::FieldReader;
#[doc = "Field `ADDRHOLD` writer - Address Hold Time"]
pub type ADDRHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HALFALE` reader - Half Cycle ALE Strobe Duration Enable"]
pub type HALFALE_R = crate::BitReader;
#[doc = "Field `HALFALE` writer - Half Cycle ALE Strobe Duration Enable"]
pub type HALFALE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Address Setup Time"]
    #[inline(always)]
    pub fn addrsetup(&self) -> ADDRSETUP_R {
        ADDRSETUP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Address Hold Time"]
    #[inline(always)]
    pub fn addrhold(&self) -> ADDRHOLD_R {
        ADDRHOLD_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 28 - Half Cycle ALE Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfale(&self) -> HALFALE_R {
        HALFALE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn addrsetup(&mut self) -> ADDRSETUP_W<ADDRTIMING1_SPEC> {
        ADDRSETUP_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Address Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn addrhold(&mut self) -> ADDRHOLD_W<ADDRTIMING1_SPEC> {
        ADDRHOLD_W::new(self, 8)
    }
    #[doc = "Bit 28 - Half Cycle ALE Strobe Duration Enable"]
    #[inline(always)]
    #[must_use]
    pub fn halfale(&mut self) -> HALFALE_W<ADDRTIMING1_SPEC> {
        HALFALE_W::new(self, 28)
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
#[doc = "Address Timing Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrtiming1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrtiming1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDRTIMING1_SPEC;
impl crate::RegisterSpec for ADDRTIMING1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrtiming1::R`](R) reader structure"]
impl crate::Readable for ADDRTIMING1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addrtiming1::W`](W) writer structure"]
impl crate::Writable for ADDRTIMING1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDRTIMING1 to value 0x0707"]
impl crate::Resettable for ADDRTIMING1_SPEC {
    const RESET_VALUE: u32 = 0x0707;
}
