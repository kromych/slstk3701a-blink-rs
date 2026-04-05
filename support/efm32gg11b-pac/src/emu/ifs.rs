#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `VMONAVDDFALL` writer - Set VMONAVDDFALL Interrupt Flag"]
pub type VmonavddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONAVDDRISE` writer - Set VMONAVDDRISE Interrupt Flag"]
pub type VmonavddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDFALL` writer - Set VMONALTAVDDFALL Interrupt Flag"]
pub type VmonaltavddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDRISE` writer - Set VMONALTAVDDRISE Interrupt Flag"]
pub type VmonaltavddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDFALL` writer - Set VMONDVDDFALL Interrupt Flag"]
pub type VmondvddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDRISE` writer - Set VMONDVDDRISE Interrupt Flag"]
pub type VmondvddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0FALL` writer - Set VMONIO0FALL Interrupt Flag"]
pub type Vmonio0fallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0RISE` writer - Set VMONIO0RISE Interrupt Flag"]
pub type Vmonio0riseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO1FALL` writer - Set VMONIO1FALL Interrupt Flag"]
pub type Vmonio1fallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO1RISE` writer - Set VMONIO1RISE Interrupt Flag"]
pub type Vmonio1riseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5VREADY` writer - Set R5VREADY Interrupt Flag"]
pub type R5vreadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONBUVDDFALL` writer - Set VMONBUVDDFALL Interrupt Flag"]
pub type VmonbuvddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONBUVDDRISE` writer - Set VMONBUVDDRISE Interrupt Flag"]
pub type VmonbuvddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - Set PFETOVERCURRENTLIMIT Interrupt Flag"]
pub type PfetovercurrentlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - Set NFETOVERCURRENTLIMIT Interrupt Flag"]
pub type NfetovercurrentlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLPRUNNING` writer - Set DCDCLPRUNNING Interrupt Flag"]
pub type DcdclprunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLNRUNNING` writer - Set DCDCLNRUNNING Interrupt Flag"]
pub type DcdclnrunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCINBYPASS` writer - Set DCDCINBYPASS Interrupt Flag"]
pub type DcdcinbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURDY` writer - Set BURDY Interrupt Flag"]
pub type BurdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5VVSINT` writer - Set R5VVSINT Interrupt Flag"]
pub type R5vvsintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23WAKEUP` writer - Set EM23WAKEUP Interrupt Flag"]
pub type Em23wakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSCALEDONE` writer - Set VSCALEDONE Interrupt Flag"]
pub type VscaledoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP` writer - Set TEMP Interrupt Flag"]
pub type TempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOW` writer - Set TEMPLOW Interrupt Flag"]
pub type TemplowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGH` writer - Set TEMPHIGH Interrupt Flag"]
pub type TemphighW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set VMONAVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonavddfall(&mut self) -> VmonavddfallW<'_, IfsSpec> {
        VmonavddfallW::new(self, 0)
    }
    #[doc = "Bit 1 - Set VMONAVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonavddrise(&mut self) -> VmonavddriseW<'_, IfsSpec> {
        VmonavddriseW::new(self, 1)
    }
    #[doc = "Bit 2 - Set VMONALTAVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonaltavddfall(&mut self) -> VmonaltavddfallW<'_, IfsSpec> {
        VmonaltavddfallW::new(self, 2)
    }
    #[doc = "Bit 3 - Set VMONALTAVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonaltavddrise(&mut self) -> VmonaltavddriseW<'_, IfsSpec> {
        VmonaltavddriseW::new(self, 3)
    }
    #[doc = "Bit 4 - Set VMONDVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmondvddfall(&mut self) -> VmondvddfallW<'_, IfsSpec> {
        VmondvddfallW::new(self, 4)
    }
    #[doc = "Bit 5 - Set VMONDVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmondvddrise(&mut self) -> VmondvddriseW<'_, IfsSpec> {
        VmondvddriseW::new(self, 5)
    }
    #[doc = "Bit 6 - Set VMONIO0FALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio0fall(&mut self) -> Vmonio0fallW<'_, IfsSpec> {
        Vmonio0fallW::new(self, 6)
    }
    #[doc = "Bit 7 - Set VMONIO0RISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio0rise(&mut self) -> Vmonio0riseW<'_, IfsSpec> {
        Vmonio0riseW::new(self, 7)
    }
    #[doc = "Bit 8 - Set VMONIO1FALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio1fall(&mut self) -> Vmonio1fallW<'_, IfsSpec> {
        Vmonio1fallW::new(self, 8)
    }
    #[doc = "Bit 9 - Set VMONIO1RISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio1rise(&mut self) -> Vmonio1riseW<'_, IfsSpec> {
        Vmonio1riseW::new(self, 9)
    }
    #[doc = "Bit 10 - Set R5VREADY Interrupt Flag"]
    #[inline(always)]
    pub fn r5vready(&mut self) -> R5vreadyW<'_, IfsSpec> {
        R5vreadyW::new(self, 10)
    }
    #[doc = "Bit 12 - Set VMONBUVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonbuvddfall(&mut self) -> VmonbuvddfallW<'_, IfsSpec> {
        VmonbuvddfallW::new(self, 12)
    }
    #[doc = "Bit 13 - Set VMONBUVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonbuvddrise(&mut self) -> VmonbuvddriseW<'_, IfsSpec> {
        VmonbuvddriseW::new(self, 13)
    }
    #[doc = "Bit 16 - Set PFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&mut self) -> PfetovercurrentlimitW<'_, IfsSpec> {
        PfetovercurrentlimitW::new(self, 16)
    }
    #[doc = "Bit 17 - Set NFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&mut self) -> NfetovercurrentlimitW<'_, IfsSpec> {
        NfetovercurrentlimitW::new(self, 17)
    }
    #[doc = "Bit 18 - Set DCDCLPRUNNING Interrupt Flag"]
    #[inline(always)]
    pub fn dcdclprunning(&mut self) -> DcdclprunningW<'_, IfsSpec> {
        DcdclprunningW::new(self, 18)
    }
    #[doc = "Bit 19 - Set DCDCLNRUNNING Interrupt Flag"]
    #[inline(always)]
    pub fn dcdclnrunning(&mut self) -> DcdclnrunningW<'_, IfsSpec> {
        DcdclnrunningW::new(self, 19)
    }
    #[doc = "Bit 20 - Set DCDCINBYPASS Interrupt Flag"]
    #[inline(always)]
    pub fn dcdcinbypass(&mut self) -> DcdcinbypassW<'_, IfsSpec> {
        DcdcinbypassW::new(self, 20)
    }
    #[doc = "Bit 22 - Set BURDY Interrupt Flag"]
    #[inline(always)]
    pub fn burdy(&mut self) -> BurdyW<'_, IfsSpec> {
        BurdyW::new(self, 22)
    }
    #[doc = "Bit 23 - Set R5VVSINT Interrupt Flag"]
    #[inline(always)]
    pub fn r5vvsint(&mut self) -> R5vvsintW<'_, IfsSpec> {
        R5vvsintW::new(self, 23)
    }
    #[doc = "Bit 24 - Set EM23WAKEUP Interrupt Flag"]
    #[inline(always)]
    pub fn em23wakeup(&mut self) -> Em23wakeupW<'_, IfsSpec> {
        Em23wakeupW::new(self, 24)
    }
    #[doc = "Bit 25 - Set VSCALEDONE Interrupt Flag"]
    #[inline(always)]
    pub fn vscaledone(&mut self) -> VscaledoneW<'_, IfsSpec> {
        VscaledoneW::new(self, 25)
    }
    #[doc = "Bit 29 - Set TEMP Interrupt Flag"]
    #[inline(always)]
    pub fn temp(&mut self) -> TempW<'_, IfsSpec> {
        TempW::new(self, 29)
    }
    #[doc = "Bit 30 - Set TEMPLOW Interrupt Flag"]
    #[inline(always)]
    pub fn templow(&mut self) -> TemplowW<'_, IfsSpec> {
        TemplowW::new(self, 30)
    }
    #[doc = "Bit 31 - Set TEMPHIGH Interrupt Flag"]
    #[inline(always)]
    pub fn temphigh(&mut self) -> TemphighW<'_, IfsSpec> {
        TemphighW::new(self, 31)
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
