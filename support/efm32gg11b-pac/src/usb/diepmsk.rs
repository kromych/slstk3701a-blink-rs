#[doc = "Register `DIEPMSK` reader"]
pub type R = crate::R<DIEPMSK_SPEC>;
#[doc = "Register `DIEPMSK` writer"]
pub type W = crate::W<DIEPMSK_SPEC>;
#[doc = "Field `XFERCOMPLMSK` reader - Transfer Completed Interrupt Mask"]
pub type XFERCOMPLMSK_R = crate::BitReader;
#[doc = "Field `XFERCOMPLMSK` writer - Transfer Completed Interrupt Mask"]
pub type XFERCOMPLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLDMSK` reader - Endpoint Disabled Interrupt Mask"]
pub type EPDISBLDMSK_R = crate::BitReader;
#[doc = "Field `EPDISBLDMSK` writer - Endpoint Disabled Interrupt Mask"]
pub type EPDISBLDMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERRMSK` reader - AHB Error Mask"]
pub type AHBERRMSK_R = crate::BitReader;
#[doc = "Field `AHBERRMSK` writer - AHB Error Mask"]
pub type AHBERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTMSK` reader - Timeout Condition Mask"]
pub type TIMEOUTMSK_R = crate::BitReader;
#[doc = "Field `TIMEOUTMSK` writer - Timeout Condition Mask"]
pub type TIMEOUTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNTXFEMPMSK` reader - IN Token Received When TxFIFO Empty Mask"]
pub type INTKNTXFEMPMSK_R = crate::BitReader;
#[doc = "Field `INTKNTXFEMPMSK` writer - IN Token Received When TxFIFO Empty Mask"]
pub type INTKNTXFEMPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNEPMISMSK` reader - IN Token received with EP Mismatch Mask"]
pub type INTKNEPMISMSK_R = crate::BitReader;
#[doc = "Field `INTKNEPMISMSK` writer - IN Token received with EP Mismatch Mask"]
pub type INTKNEPMISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNAKEFFMSK` reader - IN Endpoint NAK Effective Mask"]
pub type INEPNAKEFFMSK_R = crate::BitReader;
#[doc = "Field `INEPNAKEFFMSK` writer - IN Endpoint NAK Effective Mask"]
pub type INEPNAKEFFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOUNDRNMSK` reader - Fifo Underrun Mask"]
pub type TXFIFOUNDRNMSK_R = crate::BitReader;
#[doc = "Field `TXFIFOUNDRNMSK` writer - Fifo Underrun Mask"]
pub type TXFIFOUNDRNMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMSK` reader - NAK interrupt Mask"]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAK interrupt Mask"]
pub type NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbldmsk(&self) -> EPDISBLDMSK_R {
        EPDISBLDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout Condition Mask"]
    #[inline(always)]
    pub fn timeoutmsk(&self) -> TIMEOUTMSK_R {
        TIMEOUTMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn intkntxfempmsk(&self) -> INTKNTXFEMPMSK_R {
        INTKNTXFEMPMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN Token received with EP Mismatch Mask"]
    #[inline(always)]
    pub fn intknepmismsk(&self) -> INTKNEPMISMSK_R {
        INTKNEPMISMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask"]
    #[inline(always)]
    pub fn inepnakeffmsk(&self) -> INEPNAKEFFMSK_R {
        INEPNAKEFFMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Fifo Underrun Mask"]
    #[inline(always)]
    pub fn txfifoundrnmsk(&self) -> TXFIFOUNDRNMSK_R {
        TXFIFOUNDRNMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfercomplmsk(&mut self) -> XFERCOMPLMSK_W<DIEPMSK_SPEC> {
        XFERCOMPLMSK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbldmsk(&mut self) -> EPDISBLDMSK_W<DIEPMSK_SPEC> {
        EPDISBLDMSK_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ahberrmsk(&mut self) -> AHBERRMSK_W<DIEPMSK_SPEC> {
        AHBERRMSK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout Condition Mask"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutmsk(&mut self) -> TIMEOUTMSK_W<DIEPMSK_SPEC> {
        TIMEOUTMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO Empty Mask"]
    #[inline(always)]
    #[must_use]
    pub fn intkntxfempmsk(&mut self) -> INTKNTXFEMPMSK_W<DIEPMSK_SPEC> {
        INTKNTXFEMPMSK_W::new(self, 4)
    }
    #[doc = "Bit 5 - IN Token received with EP Mismatch Mask"]
    #[inline(always)]
    #[must_use]
    pub fn intknepmismsk(&mut self) -> INTKNEPMISMSK_W<DIEPMSK_SPEC> {
        INTKNEPMISMSK_W::new(self, 5)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask"]
    #[inline(always)]
    #[must_use]
    pub fn inepnakeffmsk(&mut self) -> INEPNAKEFFMSK_W<DIEPMSK_SPEC> {
        INEPNAKEFFMSK_W::new(self, 6)
    }
    #[doc = "Bit 8 - Fifo Underrun Mask"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoundrnmsk(&mut self) -> TXFIFOUNDRNMSK_W<DIEPMSK_SPEC> {
        TXFIFOUNDRNMSK_W::new(self, 8)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<DIEPMSK_SPEC> {
        NAKMSK_W::new(self, 13)
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
#[doc = "Device IN Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPMSK_SPEC;
impl crate::RegisterSpec for DIEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepmsk::R`](R) reader structure"]
impl crate::Readable for DIEPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepmsk::W`](W) writer structure"]
impl crate::Writable for DIEPMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPMSK to value 0"]
impl crate::Resettable for DIEPMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
