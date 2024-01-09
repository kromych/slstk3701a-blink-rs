#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `ADDRFAULTEN` reader - Invalid Address Bus Fault Response Enable"]
pub type ADDRFAULTEN_R = crate::BitReader;
#[doc = "Field `ADDRFAULTEN` writer - Invalid Address Bus Fault Response Enable"]
pub type ADDRFAULTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDISFAULTEN` reader - Clock-disabled Bus Fault Response Enable"]
pub type CLKDISFAULTEN_R = crate::BitReader;
#[doc = "Field `CLKDISFAULTEN` writer - Clock-disabled Bus Fault Response Enable"]
pub type CLKDISFAULTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPONDEMAND` reader - Power Up on Demand During Wake Up"]
pub type PWRUPONDEMAND_R = crate::BitReader;
#[doc = "Field `PWRUPONDEMAND` writer - Power Up on Demand During Wake Up"]
pub type PWRUPONDEMAND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFCREADCLEAR` reader - IFC Read Clears IF"]
pub type IFCREADCLEAR_R = crate::BitReader;
#[doc = "Field `IFCREADCLEAR` writer - IFC Read Clears IF"]
pub type IFCREADCLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTFAULTEN` reader - Timeout Bus Fault Response Enable"]
pub type TIMEOUTFAULTEN_R = crate::BitReader;
#[doc = "Field `TIMEOUTFAULTEN` writer - Timeout Bus Fault Response Enable"]
pub type TIMEOUTFAULTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMECCERRFAULTEN` reader - Two Bit ECC Error Bus Fault Response Enable"]
pub type RAMECCERRFAULTEN_R = crate::BitReader;
#[doc = "Field `RAMECCERRFAULTEN` writer - Two Bit ECC Error Bus Fault Response Enable"]
pub type RAMECCERRFAULTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBIFAULTEN` reader - EBI Bus Fault Response Enable"]
pub type EBIFAULTEN_R = crate::BitReader;
#[doc = "Field `EBIFAULTEN` writer - EBI Bus Fault Response Enable"]
pub type EBIFAULTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITMODE` reader - Peripheral Access Wait Mode"]
pub type WAITMODE_R = crate::BitReader;
#[doc = "Field `WAITMODE` writer - Peripheral Access Wait Mode"]
pub type WAITMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enable"]
    #[inline(always)]
    pub fn addrfaulten(&self) -> ADDRFAULTEN_R {
        ADDRFAULTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock-disabled Bus Fault Response Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&self) -> CLKDISFAULTEN_R {
        CLKDISFAULTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Up on Demand During Wake Up"]
    #[inline(always)]
    pub fn pwrupondemand(&self) -> PWRUPONDEMAND_R {
        PWRUPONDEMAND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IFC Read Clears IF"]
    #[inline(always)]
    pub fn ifcreadclear(&self) -> IFCREADCLEAR_R {
        IFCREADCLEAR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout Bus Fault Response Enable"]
    #[inline(always)]
    pub fn timeoutfaulten(&self) -> TIMEOUTFAULTEN_R {
        TIMEOUTFAULTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Two Bit ECC Error Bus Fault Response Enable"]
    #[inline(always)]
    pub fn rameccerrfaulten(&self) -> RAMECCERRFAULTEN_R {
        RAMECCERRFAULTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EBI Bus Fault Response Enable"]
    #[inline(always)]
    pub fn ebifaulten(&self) -> EBIFAULTEN_R {
        EBIFAULTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral Access Wait Mode"]
    #[inline(always)]
    pub fn waitmode(&self) -> WAITMODE_R {
        WAITMODE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enable"]
    #[inline(always)]
    #[must_use]
    pub fn addrfaulten(&mut self) -> ADDRFAULTEN_W<CTRL_SPEC> {
        ADDRFAULTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock-disabled Bus Fault Response Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkdisfaulten(&mut self) -> CLKDISFAULTEN_W<CTRL_SPEC> {
        CLKDISFAULTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Power Up on Demand During Wake Up"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupondemand(&mut self) -> PWRUPONDEMAND_W<CTRL_SPEC> {
        PWRUPONDEMAND_W::new(self, 2)
    }
    #[doc = "Bit 3 - IFC Read Clears IF"]
    #[inline(always)]
    #[must_use]
    pub fn ifcreadclear(&mut self) -> IFCREADCLEAR_W<CTRL_SPEC> {
        IFCREADCLEAR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timeout Bus Fault Response Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutfaulten(&mut self) -> TIMEOUTFAULTEN_W<CTRL_SPEC> {
        TIMEOUTFAULTEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Two Bit ECC Error Bus Fault Response Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rameccerrfaulten(&mut self) -> RAMECCERRFAULTEN_W<CTRL_SPEC> {
        RAMECCERRFAULTEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - EBI Bus Fault Response Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ebifaulten(&mut self) -> EBIFAULTEN_W<CTRL_SPEC> {
        EBIFAULTEN_W::new(self, 6)
    }
    #[doc = "Bit 12 - Peripheral Access Wait Mode"]
    #[inline(always)]
    #[must_use]
    pub fn waitmode(&mut self) -> WAITMODE_W<CTRL_SPEC> {
        WAITMODE_W::new(self, 12)
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
#[doc = "Memory System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x21"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x21;
}
