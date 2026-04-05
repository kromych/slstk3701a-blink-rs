#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `VBUSDETH` writer - Set VBUSDETH Interrupt Flag"]
pub type VbusdethW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSDETL` writer - Set VBUSDETL Interrupt Flag"]
pub type VbusdetlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` writer - Set ERR Interrupt Flag"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCD` writer - Set DCD Interrupt Flag"]
pub type DcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD` writer - Set PD Interrupt Flag"]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD` writer - Set SD Interrupt Flag"]
pub type SdW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set VBUSDETH Interrupt Flag"]
    #[inline(always)]
    pub fn vbusdeth(&mut self) -> VbusdethW<'_, IfsSpec> {
        VbusdethW::new(self, 0)
    }
    #[doc = "Bit 1 - Set VBUSDETL Interrupt Flag"]
    #[inline(always)]
    pub fn vbusdetl(&mut self) -> VbusdetlW<'_, IfsSpec> {
        VbusdetlW::new(self, 1)
    }
    #[doc = "Bit 8 - Set ERR Interrupt Flag"]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<'_, IfsSpec> {
        ErrW::new(self, 8)
    }
    #[doc = "Bit 9 - Set DCD Interrupt Flag"]
    #[inline(always)]
    pub fn dcd(&mut self) -> DcdW<'_, IfsSpec> {
        DcdW::new(self, 9)
    }
    #[doc = "Bit 10 - Set PD Interrupt Flag"]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<'_, IfsSpec> {
        PdW::new(self, 10)
    }
    #[doc = "Bit 11 - Set SD Interrupt Flag"]
    #[inline(always)]
    pub fn sd(&mut self) -> SdW<'_, IfsSpec> {
        SdW::new(self, 11)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
