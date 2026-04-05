#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `VBUSDETH` reader - VBUSDETH Interrupt Enable"]
pub type VbusdethR = crate::BitReader;
#[doc = "Field `VBUSDETH` writer - VBUSDETH Interrupt Enable"]
pub type VbusdethW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSDETL` reader - VBUSDETL Interrupt Enable"]
pub type VbusdetlR = crate::BitReader;
#[doc = "Field `VBUSDETL` writer - VBUSDETL Interrupt Enable"]
pub type VbusdetlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - ERR Interrupt Enable"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - ERR Interrupt Enable"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCD` reader - DCD Interrupt Enable"]
pub type DcdR = crate::BitReader;
#[doc = "Field `DCD` writer - DCD Interrupt Enable"]
pub type DcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD` reader - PD Interrupt Enable"]
pub type PdR = crate::BitReader;
#[doc = "Field `PD` writer - PD Interrupt Enable"]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD` reader - SD Interrupt Enable"]
pub type SdR = crate::BitReader;
#[doc = "Field `SD` writer - SD Interrupt Enable"]
pub type SdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VBUSDETH Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdeth(&self) -> VbusdethR {
        VbusdethR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBUSDETL Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdetl(&self) -> VbusdetlR {
        VbusdetlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - ERR Interrupt Enable"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DCD Interrupt Enable"]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PD Interrupt Enable"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD Interrupt Enable"]
    #[inline(always)]
    pub fn sd(&self) -> SdR {
        SdR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUSDETH Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdeth(&mut self) -> VbusdethW<'_, IenSpec> {
        VbusdethW::new(self, 0)
    }
    #[doc = "Bit 1 - VBUSDETL Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdetl(&mut self) -> VbusdetlW<'_, IenSpec> {
        VbusdetlW::new(self, 1)
    }
    #[doc = "Bit 8 - ERR Interrupt Enable"]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<'_, IenSpec> {
        ErrW::new(self, 8)
    }
    #[doc = "Bit 9 - DCD Interrupt Enable"]
    #[inline(always)]
    pub fn dcd(&mut self) -> DcdW<'_, IenSpec> {
        DcdW::new(self, 9)
    }
    #[doc = "Bit 10 - PD Interrupt Enable"]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<'_, IenSpec> {
        PdW::new(self, 10)
    }
    #[doc = "Bit 11 - SD Interrupt Enable"]
    #[inline(always)]
    pub fn sd(&mut self) -> SdW<'_, IenSpec> {
        SdW::new(self, 11)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
