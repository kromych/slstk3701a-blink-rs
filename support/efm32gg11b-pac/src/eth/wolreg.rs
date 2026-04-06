#[doc = "Register `WOLREG` reader"]
pub type R = crate::R<WolregSpec>;
#[doc = "Register `WOLREG` writer"]
pub type W = crate::W<WolregSpec>;
#[doc = "Field `ADDR` reader - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WOLMASK0` reader - Wake on LAN magic packet event enable"]
pub type Wolmask0R = crate::BitReader;
#[doc = "Field `WOLMASK0` writer - Wake on LAN magic packet event enable"]
pub type Wolmask0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WOLMASK1` reader - Wake on LAN ARP request event enable"]
pub type Wolmask1R = crate::BitReader;
#[doc = "Field `WOLMASK1` writer - Wake on LAN ARP request event enable"]
pub type Wolmask1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WOLMASK2` reader - Wake on LAN specific address register 1 event enable"]
pub type Wolmask2R = crate::BitReader;
#[doc = "Field `WOLMASK2` writer - Wake on LAN specific address register 1 event enable"]
pub type Wolmask2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WOLMASK3` reader - Wake on LAN multicast hash event enable"]
pub type Wolmask3R = crate::BitReader;
#[doc = "Field `WOLMASK3` writer - Wake on LAN multicast hash event enable"]
pub type Wolmask3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Wake on LAN magic packet event enable"]
    #[inline(always)]
    pub fn wolmask0(&self) -> Wolmask0R {
        Wolmask0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wake on LAN ARP request event enable"]
    #[inline(always)]
    pub fn wolmask1(&self) -> Wolmask1R {
        Wolmask1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wake on LAN specific address register 1 event enable"]
    #[inline(always)]
    pub fn wolmask2(&self) -> Wolmask2R {
        Wolmask2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Wake on LAN multicast hash event enable"]
    #[inline(always)]
    pub fn wolmask3(&self) -> Wolmask3R {
        Wolmask3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, WolregSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 16 - Wake on LAN magic packet event enable"]
    #[inline(always)]
    pub fn wolmask0(&mut self) -> Wolmask0W<'_, WolregSpec> {
        Wolmask0W::new(self, 16)
    }
    #[doc = "Bit 17 - Wake on LAN ARP request event enable"]
    #[inline(always)]
    pub fn wolmask1(&mut self) -> Wolmask1W<'_, WolregSpec> {
        Wolmask1W::new(self, 17)
    }
    #[doc = "Bit 18 - Wake on LAN specific address register 1 event enable"]
    #[inline(always)]
    pub fn wolmask2(&mut self) -> Wolmask2W<'_, WolregSpec> {
        Wolmask2W::new(self, 18)
    }
    #[doc = "Bit 19 - Wake on LAN multicast hash event enable"]
    #[inline(always)]
    pub fn wolmask3(&mut self) -> Wolmask3W<'_, WolregSpec> {
        Wolmask3W::new(self, 19)
    }
}
#[doc = "Wake on LAN Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wolreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wolreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WolregSpec;
impl crate::RegisterSpec for WolregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wolreg::R`](R) reader structure"]
impl crate::Readable for WolregSpec {}
#[doc = "`write(|w| ..)` method takes [`wolreg::W`](W) writer structure"]
impl crate::Writable for WolregSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WOLREG to value 0"]
impl crate::Resettable for WolregSpec {}
