#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `TXC` writer - Set TXC Interrupt Flag"]
pub type TXC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` writer - Set RXOF Interrupt Flag"]
pub type RXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Set RXUF Interrupt Flag"]
pub type RXUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Set TXOF Interrupt Flag"]
pub type TXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` writer - Set PERR Interrupt Flag"]
pub type PERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` writer - Set FERR Interrupt Flag"]
pub type FERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` writer - Set MPAF Interrupt Flag"]
pub type MPAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTF` writer - Set STARTF Interrupt Flag"]
pub type STARTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIGF` writer - Set SIGF Interrupt Flag"]
pub type SIGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set TXC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<IFS_SPEC> {
        TXC_W::new(self, 0)
    }
    #[doc = "Bit 3 - Set RXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RXOF_W<IFS_SPEC> {
        RXOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set RXUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<IFS_SPEC> {
        RXUF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set TXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<IFS_SPEC> {
        TXOF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set PERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<IFS_SPEC> {
        PERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set FERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<IFS_SPEC> {
        FERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set MPAF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mpaf(&mut self) -> MPAF_W<IFS_SPEC> {
        MPAF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set STARTF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn startf(&mut self) -> STARTF_W<IFS_SPEC> {
        STARTF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set SIGF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sigf(&mut self) -> SIGF_W<IFS_SPEC> {
        SIGF_W::new(self, 10)
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
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: u32 = 0;
}
