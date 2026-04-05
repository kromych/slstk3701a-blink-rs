#[doc = "Register `GDFIFOCFG` reader"]
pub type R = crate::R<GdfifocfgSpec>;
#[doc = "Register `GDFIFOCFG` writer"]
pub type W = crate::W<GdfifocfgSpec>;
#[doc = "Field `GDFIFOCFG` reader - "]
pub type GdfifocfgR = crate::FieldReader<u16>;
#[doc = "Field `GDFIFOCFG` writer - "]
pub type GdfifocfgW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EPINFOBASEADDR` reader - "]
pub type EpinfobaseaddrR = crate::FieldReader<u16>;
#[doc = "Field `EPINFOBASEADDR` writer - "]
pub type EpinfobaseaddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gdfifocfg(&self) -> GdfifocfgR {
        GdfifocfgR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn epinfobaseaddr(&self) -> EpinfobaseaddrR {
        EpinfobaseaddrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gdfifocfg(&mut self) -> GdfifocfgW<'_, GdfifocfgSpec> {
        GdfifocfgW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn epinfobaseaddr(&mut self) -> EpinfobaseaddrW<'_, GdfifocfgSpec> {
        EpinfobaseaddrW::new(self, 16)
    }
}
#[doc = "Global DFIFO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdfifocfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdfifocfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdfifocfgSpec;
impl crate::RegisterSpec for GdfifocfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdfifocfg::R`](R) reader structure"]
impl crate::Readable for GdfifocfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gdfifocfg::W`](W) writer structure"]
impl crate::Writable for GdfifocfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GDFIFOCFG to value 0x01f2_0200"]
impl crate::Resettable for GdfifocfgSpec {
    const RESET_VALUE: u32 = 0x01f2_0200;
}
