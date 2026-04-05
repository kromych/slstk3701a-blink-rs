#[doc = "Register `POLLINGFLASHSTATUS` reader"]
pub type R = crate::R<PollingflashstatusSpec>;
#[doc = "Register `POLLINGFLASHSTATUS` writer"]
pub type W = crate::W<PollingflashstatusSpec>;
#[doc = "Field `DEVICESTATUS` reader - Device Status"]
pub type DevicestatusR = crate::FieldReader;
#[doc = "Field `DEVICESTATUSVALID` reader - Device Status Valid"]
pub type DevicestatusvalidR = crate::BitReader;
#[doc = "Field `DEVICESTATUSNBDUMMY` reader - Auto-polling Dummy Cycles"]
pub type DevicestatusnbdummyR = crate::FieldReader;
#[doc = "Field `DEVICESTATUSNBDUMMY` writer - Auto-polling Dummy Cycles"]
pub type DevicestatusnbdummyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Device Status"]
    #[inline(always)]
    pub fn devicestatus(&self) -> DevicestatusR {
        DevicestatusR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Device Status Valid"]
    #[inline(always)]
    pub fn devicestatusvalid(&self) -> DevicestatusvalidR {
        DevicestatusvalidR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Auto-polling Dummy Cycles"]
    #[inline(always)]
    pub fn devicestatusnbdummy(&self) -> DevicestatusnbdummyR {
        DevicestatusnbdummyR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Auto-polling Dummy Cycles"]
    #[inline(always)]
    pub fn devicestatusnbdummy(&mut self) -> DevicestatusnbdummyW<'_, PollingflashstatusSpec> {
        DevicestatusnbdummyW::new(self, 16)
    }
}
#[doc = "Polling Flash Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pollingflashstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pollingflashstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PollingflashstatusSpec;
impl crate::RegisterSpec for PollingflashstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pollingflashstatus::R`](R) reader structure"]
impl crate::Readable for PollingflashstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pollingflashstatus::W`](W) writer structure"]
impl crate::Writable for PollingflashstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POLLINGFLASHSTATUS to value 0"]
impl crate::Resettable for PollingflashstatusSpec {}
