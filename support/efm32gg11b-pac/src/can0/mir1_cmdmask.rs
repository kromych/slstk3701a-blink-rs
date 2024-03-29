#[doc = "Register `MIR1_CMDMASK` reader"]
pub type R = crate::R<MIR1_CMDMASK_SPEC>;
#[doc = "Register `MIR1_CMDMASK` writer"]
pub type W = crate::W<MIR1_CMDMASK_SPEC>;
#[doc = "Field `DATAB` reader - CC Channel Mode"]
pub type DATAB_R = crate::BitReader;
#[doc = "Field `DATAB` writer - CC Channel Mode"]
pub type DATAB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAA` reader - Access Data Bytes 0-3"]
pub type DATAA_R = crate::BitReader;
#[doc = "Field `DATAA` writer - Access Data Bytes 0-3"]
pub type DATAA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRQSTNEWDAT` reader - Transmission Request Bit/ New Data Bit"]
pub type TXRQSTNEWDAT_R = crate::BitReader;
#[doc = "Field `TXRQSTNEWDAT` writer - Transmission Request Bit/ New Data Bit"]
pub type TXRQSTNEWDAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRINTPND` reader - Clear Interrupt Pending Bit"]
pub type CLRINTPND_R = crate::BitReader;
#[doc = "Field `CLRINTPND` writer - Clear Interrupt Pending Bit"]
pub type CLRINTPND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTROL` reader - Access Control Bits"]
pub type CONTROL_R = crate::BitReader;
#[doc = "Field `CONTROL` writer - Access Control Bits"]
pub type CONTROL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBACC` reader - Access Arbitration Bits"]
pub type ARBACC_R = crate::BitReader;
#[doc = "Field `ARBACC` writer - Access Arbitration Bits"]
pub type ARBACC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASKACC` reader - Access Mask Bits"]
pub type MASKACC_R = crate::BitReader;
#[doc = "Field `MASKACC` writer - Access Mask Bits"]
pub type MASKACC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRRD` reader - Write/Read RAM"]
pub type WRRD_R = crate::BitReader;
#[doc = "Field `WRRD` writer - Write/Read RAM"]
pub type WRRD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CC Channel Mode"]
    #[inline(always)]
    pub fn datab(&self) -> DATAB_R {
        DATAB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Access Data Bytes 0-3"]
    #[inline(always)]
    pub fn dataa(&self) -> DATAA_R {
        DATAA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Request Bit/ New Data Bit"]
    #[inline(always)]
    pub fn txrqstnewdat(&self) -> TXRQSTNEWDAT_R {
        TXRQSTNEWDAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit"]
    #[inline(always)]
    pub fn clrintpnd(&self) -> CLRINTPND_R {
        CLRINTPND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Access Control Bits"]
    #[inline(always)]
    pub fn control(&self) -> CONTROL_R {
        CONTROL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Access Arbitration Bits"]
    #[inline(always)]
    pub fn arbacc(&self) -> ARBACC_R {
        ARBACC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Access Mask Bits"]
    #[inline(always)]
    pub fn maskacc(&self) -> MASKACC_R {
        MASKACC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write/Read RAM"]
    #[inline(always)]
    pub fn wrrd(&self) -> WRRD_R {
        WRRD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn datab(&mut self) -> DATAB_W<MIR1_CMDMASK_SPEC> {
        DATAB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Access Data Bytes 0-3"]
    #[inline(always)]
    #[must_use]
    pub fn dataa(&mut self) -> DATAA_W<MIR1_CMDMASK_SPEC> {
        DATAA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmission Request Bit/ New Data Bit"]
    #[inline(always)]
    #[must_use]
    pub fn txrqstnewdat(&mut self) -> TXRQSTNEWDAT_W<MIR1_CMDMASK_SPEC> {
        TXRQSTNEWDAT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit"]
    #[inline(always)]
    #[must_use]
    pub fn clrintpnd(&mut self) -> CLRINTPND_W<MIR1_CMDMASK_SPEC> {
        CLRINTPND_W::new(self, 3)
    }
    #[doc = "Bit 4 - Access Control Bits"]
    #[inline(always)]
    #[must_use]
    pub fn control(&mut self) -> CONTROL_W<MIR1_CMDMASK_SPEC> {
        CONTROL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Access Arbitration Bits"]
    #[inline(always)]
    #[must_use]
    pub fn arbacc(&mut self) -> ARBACC_W<MIR1_CMDMASK_SPEC> {
        ARBACC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Access Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn maskacc(&mut self) -> MASKACC_W<MIR1_CMDMASK_SPEC> {
        MASKACC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write/Read RAM"]
    #[inline(always)]
    #[must_use]
    pub fn wrrd(&mut self) -> WRRD_W<MIR1_CMDMASK_SPEC> {
        WRRD_W::new(self, 7)
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
#[doc = "Interface Command Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_cmdmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_cmdmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIR1_CMDMASK_SPEC;
impl crate::RegisterSpec for MIR1_CMDMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir1_cmdmask::R`](R) reader structure"]
impl crate::Readable for MIR1_CMDMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mir1_cmdmask::W`](W) writer structure"]
impl crate::Writable for MIR1_CMDMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIR1_CMDMASK to value 0"]
impl crate::Resettable for MIR1_CMDMASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
