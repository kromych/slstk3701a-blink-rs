#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `VBUSDETH` writer - Set VBUSDETH Interrupt Flag"]
pub type VBUSDETH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSDETL` writer - Set VBUSDETL Interrupt Flag"]
pub type VBUSDETL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` writer - Set ERR Interrupt Flag"]
pub type ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCD` writer - Set DCD Interrupt Flag"]
pub type DCD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD` writer - Set PD Interrupt Flag"]
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD` writer - Set SD Interrupt Flag"]
pub type SD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set VBUSDETH Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vbusdeth(&mut self) -> VBUSDETH_W<IFS_SPEC> {
        VBUSDETH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set VBUSDETL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vbusdetl(&mut self) -> VBUSDETL_W<IFS_SPEC> {
        VBUSDETL_W::new(self, 1)
    }
    #[doc = "Bit 8 - Set ERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<IFS_SPEC> {
        ERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set DCD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcd(&mut self) -> DCD_W<IFS_SPEC> {
        DCD_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set PD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<IFS_SPEC> {
        PD_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set SD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sd(&mut self) -> SD_W<IFS_SPEC> {
        SD_W::new(self, 11)
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
